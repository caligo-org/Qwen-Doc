//! `mpdocbench-score`: a Rust port of the MPDocBench end-to-end scorer
//! (`pdf_validation.py --config configs/end2end.yaml` with `end2end_dataset`,
//! `quick_match` and the end2end_eval task). Same config, same inputs, same output
//! files under `<out>/result/<save_name>/`.

mod cdm;
mod dataset;
mod extract;
mod lev;
mod lsap;
mod matching;
mod metrics;
mod pairs;
mod pdnum;
mod preprocess;
mod py;
mod pyset;
mod pyval;

use dataset::{DocResult, Opts};
use pyval::{Map, V};
use rayon::prelude::*;
use std::path::PathBuf;
use std::time::Instant;

const USAGE: &str = "usage: mpdocbench-score --config CONFIG.yaml [options]

Scores Markdown predictions against MPDocBench-format ground truth exactly as
`python pdf_validation.py --config CONFIG.yaml` does, writing the same files to
<out>/result/<save_name>/ (save_name = basename of the prediction data_path).

options:
  --out DIR          directory that receives result/ (default: current directory,
                     as the Python scorer writes ./result relative to its cwd)
  --variant V        mpdocbench (default; this toolkit) or marathon (the toolkit with
                     MarathonDocBench's evaluator patch: utils/mpdocbench_patch)
  --jobs N           documents matched in parallel (default: MPDOC_EVAL_NPROC or all cores)
  --cdm-jobs N       CDM worker processes, one thread each (default: MPDOC_CDM_NPROC or all cores)
  --toolkit DIR      toolkit root whose metrics/cdm/modules/tokenize_latex JS (KaTeX) CDM runs
                     with node (default: the MPDocBench/ directory this binary was built from)
  --teds-zero FILE   tables to score 0, one \"<element>/<img_id>_<gt_idx>\" per line (e.g.
                     table/doc.pdf_[3]), to reproduce a Python run (MarathonDocBench patch) in
                     which MPDOC_TEDS_TIMEOUT cut those tables off
  --export-pairs DIR  also write DIR/<element>_pairs.jsonl for text_block, merged_text_block,
                     table and merged_table: one line per scored pair with the gold elements it
                     contains (anno_ids, pages, polys), the compared strings and its score
  --simple-match FILE  documents (image_path basenames) to match with match_gt2pred_simple, to
                     reproduce a Python run in which the 30 s func_timeout fired for them
";

struct Args {
    config: PathBuf,
    out: PathBuf,
    marathon: bool,
    jobs: usize,
    cdm_jobs: usize,
    toolkit: Option<PathBuf>,
    teds_zero: std::collections::HashSet<String>,
    simple_match: std::collections::HashSet<String>,
    export_pairs: Option<PathBuf>,
}

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok().and_then(|v| v.trim().parse().ok()).filter(|&n| n > 0)
}

fn read_lines(path: &str) -> std::collections::HashSet<String> {
    std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("{}: {}", path, e))
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect()
}

fn parse_args() -> Args {
    let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let mut a = Args {
        config: PathBuf::new(),
        out: std::env::current_dir().unwrap(),
        marathon: false,
        jobs: env_usize("MPDOC_EVAL_NPROC").unwrap_or(cores),
        cdm_jobs: env_usize("MPDOC_CDM_NPROC").unwrap_or(cores),
        toolkit: None,
        teds_zero: Default::default(),
        simple_match: Default::default(),
        export_pairs: None,
    };
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut i = 0;
    let val = |i: &mut usize| -> String {
        *i += 1;
        argv.get(*i).cloned().unwrap_or_else(|| {
            eprint!("{}", USAGE);
            std::process::exit(2)
        })
    };
    while i < argv.len() {
        match argv[i].as_str() {
            "--config" | "-c" => a.config = PathBuf::from(val(&mut i)),
            "--out" => a.out = PathBuf::from(val(&mut i)),
            "--variant" => {
                a.marathon = match val(&mut i).as_str() {
                    "marathon" => true,
                    "mpdocbench" => false,
                    v => {
                        eprintln!("unknown variant {}", v);
                        std::process::exit(2)
                    }
                }
            }
            "--jobs" => a.jobs = val(&mut i).parse().expect("--jobs N"),
            "--cdm-jobs" => a.cdm_jobs = val(&mut i).parse().expect("--cdm-jobs N"),
            "--toolkit" => a.toolkit = Some(PathBuf::from(val(&mut i))),
            "--teds-zero" => a.teds_zero = read_lines(&val(&mut i)),
            "--simple-match" => a.simple_match = read_lines(&val(&mut i)),
            "--export-pairs" => a.export_pairs = Some(PathBuf::from(val(&mut i))),
            "-h" | "--help" => {
                print!("{}", USAGE);
                std::process::exit(0)
            }
            other => {
                eprintln!("unknown argument {}\n{}", other, USAGE);
                std::process::exit(2)
            }
        }
        i += 1;
    }
    if a.config.as_os_str().is_empty() {
        eprint!("{}", USAGE);
        std::process::exit(2);
    }
    a
}

fn yaml_to_v(y: serde_yaml::Value) -> V {
    let j: serde_json::Value = serde_json::to_value(y).expect("config to json");
    pyval::from_serde(j)
}

/// Python `os.path.basename`.
fn basename(p: &str) -> String {
    p.rsplit('/').next().unwrap_or(p).to_string()
}

fn main() {
    let args = parse_args();
    let t0 = Instant::now();
    let cfg_text = std::fs::read_to_string(&args.config).unwrap_or_else(|e| panic!("{}: {}", args.config.display(), e));
    let cfg = yaml_to_v(serde_yaml::from_str(&cfg_text).expect("YAML config"));
    if let Some(t) = &args.toolkit {
        std::env::set_var("MPD_CDM_TOKENIZE_DIR", t.join("metrics/cdm/modules/tokenize_latex"));
    }
    rayon::ThreadPoolBuilder::new().num_threads(args.jobs.max(1)).build_global().unwrap();

    for (task, tcfg) in cfg.as_dict().expect("config mapping").iter() {
        if task != "end2end_eval" {
            panic!("task {} is not supported by the Rust scorer (only end2end_eval)", task);
        }
        let ds = tcfg.get("dataset");
        if ds.get("dataset_name").as_str() != Some("end2end_dataset") {
            panic!("dataset_name {} not supported (only end2end_dataset)", ds.get("dataset_name").py_repr());
        }
        let mm = ds.get("match_method");
        if !mm.is_null() && mm.as_str() != Some("quick_match") {
            panic!("match_method {} not supported (only quick_match)", mm.py_repr());
        }
        run_task(&args, tcfg, t0);
    }
}

fn run_task(args: &Args, tcfg: &V, t0: Instant) {
    let metric_keys: Vec<String> = tcfg.get("metrics").as_dict().map(|d| d.keys().cloned().collect()).unwrap_or_default();
    let wants = |ks: &[&str]| metric_keys.is_empty() || metric_keys.iter().any(|k| ks.contains(&k.as_str()));
    let opts = Opts {
        marathon: args.marathon,
        skip_quick: !wants(&["text_block", "display_formula", "reading_order"]),
        skip_merged: !wants(&["merged_text_block"]),
    };
    let ds = tcfg.get("dataset");
    let gt_path = ds.get("ground_truth").get("data_path").py_str();
    let pred_path = ds.get("prediction").get("data_path").py_str();
    let save_name = if ds.get("prediction").get("data_path").truthy() {
        basename(&pred_path)
    } else {
        basename(&gt_path).split('.').next().unwrap().to_string()
    };
    eprintln!("###### Process:  {}", save_name);
    let metrics_cfg = tcfg.get("metrics").as_dict().expect("metrics").clone();
    let required: Vec<String> = metrics_cfg.keys().cloned().collect();
    for el in &required {
        if ["table_relation", "text_relation"].contains(&el.as_str()) {
            panic!("element {} is not supported by the Rust scorer", el);
        }
    }

    let gt_text = std::fs::read_to_string(&gt_path).unwrap_or_else(|e| panic!("{}: {}", gt_path, e));
    let gt_all = pyval::from_json(&gt_text).expect("ground truth JSON");
    drop(gt_text);
    let gt_list = gt_all.as_list().expect("ground truth list").clone();
    let filter = ds.get("filter");
    let samples: Vec<V> = if filter.truthy() {
        gt_list
            .iter()
            .filter(|s| {
                filter
                    .as_dict()
                    .unwrap()
                    .iter()
                    .all(|(k, v)| s.get("page_info").get("page_attribute").get(k).py_eq(v))
            })
            .cloned()
            .collect()
    } else {
        gt_list.to_vec()
    };
    eprintln!("[{:.1}s] loaded {} documents", t0.elapsed().as_secs_f64(), samples.len());

    let need: std::collections::HashSet<&str> = required.iter().map(|s| s.as_str()).collect();
    let only_aux = !need.is_empty() && need.iter().all(|e| ["figure", "table_relation", "text_relation"].contains(e));
    let need_figure = need.is_empty() || need.contains("figure");
    let pred_folder = PathBuf::from(&pred_path);

    // per-document matching, heaviest documents first
    let mut order: Vec<usize> = (0..samples.len()).collect();
    order.sort_by_key(|&i| std::cmp::Reverse(samples[i].get("layout_dets").py_len()));
    let done = std::sync::atomic::AtomicUsize::new(0);
    let total = samples.len();
    let mut doc_results: Vec<(usize, Option<DocResult>, f64)> = order
        .par_iter()
        .map(|&i| {
            let t = Instant::now();
            let s = &samples[i];
            let img = basename(&s.get("page_info").get("image_path").py_str());
            let r = dataset::process_doc(s, &pred_folder, opts, !only_aux, need_figure, args.simple_match.contains(&img));
            let k = done.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if k % 50 == 0 || k == total {
                eprintln!("[{:.1}s] matched {}/{}", t0.elapsed().as_secs_f64(), k, total);
            }
            (i, r, t.elapsed().as_secs_f64())
        })
        .collect();
    doc_results.sort_by_key(|x| x.0);
    eprintln!("[{:.1}s] matching done", t0.elapsed().as_secs_f64());

    // assemble the per-element lists as get_matched_elements does
    let mut plain = Vec::new();
    let mut merged_plain = Vec::new();
    let mut display = Vec::new();
    let mut html_table = Vec::new();
    let mut merged_html_table = Vec::new();
    let mut latex_table = Vec::new();
    let mut merged_latex_table = Vec::new();
    let mut order_match = Vec::new();
    let mut head_match = Vec::new();
    let mut head_inputs = Vec::new();
    let mut figure_match = Vec::new();
    for (_, r, _) in doc_results {
        let Some(r) = r else { continue };
        if let Some(f) = r.figure {
            figure_match.push(f);
        }
        let Some(m) = r.main else { continue };
        if let Some(o) = m.order {
            order_match.push(o);
        }
        head_match.push(m.head.sample.clone());
        head_inputs.push(m.head);
        plain.extend(m.plain_text);
        display.extend(m.display_formula);
        latex_table.extend(m.latex_table);
        html_table.extend(m.html_table);
        merged_html_table.extend(m.merged_html_table);
        merged_plain.extend(m.merged_plain_text);
        merged_latex_table.extend(m.merged_latex_table);
    }
    let (display, others) = dataset::split_display_formulas(display, opts);
    if !others.is_empty() && !plain.is_empty() {
        plain.extend(others);
    }
    let blank_pred = |mut t: V| {
        let keys: Vec<String> = t.as_dict().unwrap().keys().filter(|k| k.contains("pred")).cloned().collect();
        for k in keys {
            t.set(&k, V::str(""));
        }
        t.set("edit", V::Int(1));
        t
    };
    let latex_n = latex_table.len();
    html_table.extend(latex_table.into_iter().map(blank_pred));
    merged_html_table.extend(merged_latex_table.into_iter().map(blank_pred));
    // len(latex_table_match) > len(html_table_match) can no longer hold: tables are scored as html
    let _ = latex_n;
    let normalize = |v: Vec<V>| -> Vec<V> {
        v.into_par_iter()
            .map(|mut s| {
                let p = pysoup::normalized_html_table(&s.get("pred").py_str());
                let r = pysoup::normalized_html_table(&s.get("gt").py_str());
                s.set("norm_gt", V::from(r));
                s.set("norm_pred", V::from(p));
                s
            })
            .collect()
    };
    let table = normalize(html_table);
    let merged_table = normalize(merged_html_table);

    let mut elements: Vec<(&str, Vec<V>)> = vec![
        ("text_block", plain),
        ("merged_text_block", merged_plain),
        ("display_formula", display),
        ("table", table),
        ("merged_table", merged_table),
        ("reading_order", order_match),
        ("head", head_match),
        ("figure", figure_match),
    ];

    // page_info from the full ground truth
    let mut page_info = Map::new();
    let page_info_src = ds.get("ground_truth").get("page_info");
    if page_info_src.truthy() {
        panic!("ground_truth.page_info is not supported");
    }
    for p in gt_list.iter() {
        let img = basename(&p.get("page_info").get("image_path").py_str());
        page_info.insert(img, p.get("page_info").get("page_attribute").clone());
    }

    let out_dir = args.out.join("result").join(&save_name);
    std::fs::create_dir_all(&out_dir).unwrap();
    let cdm_cfg = cdm::CdmConfig { workers: args.cdm_jobs, marathon: args.marathon, base: args.out.clone() };

    let mut result_all = Map::new();
    for (element, ecfg) in metrics_cfg.iter() {
        let pos = elements
            .iter()
            .position(|(e, _)| e == element)
            .unwrap_or_else(|| panic!("KeyError: element {}", element));
        let mut samples = std::mem::take(&mut elements[pos].1);
        let prefix = out_dir.join(element).to_string_lossy().to_string();
        let mut result = Map::new();
        let mut is_list = false;
        let te = Instant::now();
        for metric in ecfg.get("metric").as_list().expect("metric list").iter() {
            let metric = metric.py_str();
            let r = match metric.as_str() {
                "Edit_dist" => {
                    is_list = false;
                    metrics::edit_dist(&mut samples, &prefix)
                }
                "TEDS" => {
                    is_list = args.marathon;
                    metrics::teds_metric(&mut samples, &prefix, element, args.marathon, &args.teds_zero)
                }
                "HeadTEDS" => {
                    is_list = false;
                    if element != "head" {
                        panic!("HeadTEDS is only supported on the head element");
                    }
                    metrics::head_teds_metric(&mut samples, &head_inputs, &prefix)
                }
                "FigureF1" => {
                    is_list = false;
                    metrics::figure_f1(&mut samples, &prefix)
                }
                "CDM" => {
                    is_list = true;
                    let root = format!("result/{}/{}/CDM", save_name, element);
                    let (s, r) = cdm::cdm_metric(&cdm_cfg, &samples, &prefix, &root);
                    samples = s;
                    r
                }
                other => panic!("metric {} is not supported by the Rust scorer", other),
            };
            if r.truthy() {
                for (k, v) in r.as_dict().unwrap().iter() {
                    result.insert(k.clone(), v.clone());
                }
            }
        }
        eprintln!("[{:.1}s] {} scored in {:.1}s", t0.elapsed().as_secs_f64(), element, te.elapsed().as_secs_f64());
        print_result(element, &result);
        let group = metrics::full_labels_results(&samples, is_list);
        let page = metrics::page_split(&samples, &page_info);
        let mut entry = Map::new();
        entry.insert("all".into(), V::from(result));
        entry.insert("group".into(), group);
        entry.insert("page".into(), page);
        if element == "figure" {
            entry.insert("figure_label".into(), metrics::figure_label_results(&samples));
        }
        result_all.insert(element.clone(), V::from(entry));
        if let Some(dir) = &args.export_pairs {
            if pairs::CATEGORIES.contains(&element.as_str()) {
                pairs::export(dir, element, &samples);
            }
        }
        metrics::write_json(&out_dir.join(format!("{}_result.json", element)), &V::list(samples));
    }
    metrics::write_json(&out_dir.join("metric_result.json"), &V::from(result_all));
    eprintln!("[{:.1}s] done: {}", t0.elapsed().as_secs_f64(), out_dir.join("metric_result.json").display());
}

fn print_result(element: &str, result: &Map) {
    if result.is_empty() {
        return;
    }
    println!("【{}】", element);
    for (metric, vals) in result {
        println!("{}:", metric);
        if let Some(d) = vals.as_dict() {
            for (k, v) in d {
                println!("  {:<20} {}", k, v.py_str());
            }
        }
    }
}
