//! A line-by-line port of the Python `apted` 1.0.3 package (`APTED(t1, t2, config)
//! .compute_edit_distance()`) with unit insert/delete costs.
//!
//! This is deliberately *not* a clean tree-edit-distance implementation: apted 1.0.3 does
//! not always return the optimal distance (its single-path functions read entries of the
//! shared `delta` matrix that still hold strategy path ids, among other quirks), and the
//! MPDocBench scores are defined by what it returns. Everything that can influence the
//! result is reproduced: the strategy computation (with its ties), the shared `delta`
//! matrix holding both path ids and distances, Python's negative list indexing, the
//! `fn`/`ft` arrays with their `-1` sentinel slot, the per-path state that persists across
//! `loop_b` calls, and the floating-point operation order of every sum and `min`.
//!
//! A Python `IndexError` (an index outside a list even after negative wrapping) makes the
//! computation return `Err(Error::Apted)`.

use std::cell::Cell;

/// Node indices of one input tree (`NodeIndexer`). Nodes are numbered in left-to-right
/// preorder; `-1` stands for the dummy `NodeInfo`s (the root's parent, `EMPTY`, the fake
/// child of `walk_up`, the fresh node `leftmost`/`rightmost` return for a leaf), whose
/// indices are all -1, size 1 and cost 0, and which are falsy.
struct Indexer {
    num: usize,
    node: Vec<usize>,
    post_ltr: Vec<isize>,
    pre_rtl: Vec<isize>,
    post_rtl: Vec<isize>,
    parent: Vec<isize>,
    children: Vec<Vec<usize>>,
    type_l: Vec<bool>,
    type_r: Vec<bool>,
    desc_sum: Vec<f64>,
    kr_sum: Vec<f64>,
    rev_kr_sum: Vec<f64>,
    size: Vec<usize>,
    lnl: Vec<isize>,
    lnr: Vec<isize>,
    lld: Vec<usize>,
    rld: Vec<usize>,
    sum_cost: Vec<f64>,
    post_ltr_info: Vec<usize>,
    pre_rtl_info: Vec<usize>,
    post_rtl_info: Vec<usize>,
    tree_size: usize,
    lchl: usize,
    rchl: usize,
}

impl Indexer {
    fn new(children: &[Vec<usize>], root: usize, num: usize) -> Indexer {
        let mut ix = Indexer {
            num,
            node: Vec::new(),
            post_ltr: Vec::new(),
            pre_rtl: Vec::new(),
            post_rtl: Vec::new(),
            parent: Vec::new(),
            children: Vec::new(),
            type_l: Vec::new(),
            type_r: Vec::new(),
            desc_sum: Vec::new(),
            kr_sum: Vec::new(),
            rev_kr_sum: Vec::new(),
            size: Vec::new(),
            lnl: Vec::new(),
            lnr: Vec::new(),
            lld: Vec::new(),
            rld: Vec::new(),
            sum_cost: Vec::new(),
            post_ltr_info: Vec::new(),
            pre_rtl_info: Vec::new(),
            post_rtl_info: Vec::new(),
            tree_size: 0,
            lchl: 0,
            rchl: 0,
        };
        ix.index_nodes(children, root);
        ix.tree_size = ix.size[0];
        ix.post_traversal_indexing();
        ix
    }

    fn add(&mut self, node: usize) -> usize {
        let p = self.node.len();
        self.node.push(node);
        self.post_ltr.push(-1);
        self.pre_rtl.push(-1);
        self.post_rtl.push(-1);
        self.parent.push(-1);
        self.children.push(Vec::new());
        self.type_l.push(false);
        self.type_r.push(false);
        self.desc_sum.push(0.0);
        self.kr_sum.push(0.0);
        self.rev_kr_sum.push(0.0);
        self.size.push(1);
        self.lnl.push(-1);
        self.lnr.push(-1);
        self.lld.push(0);
        self.rld.push(0);
        self.sum_cost.push(0.0);
        p
    }

    /// `NodeIndexer.index_nodes` (iterative; same numbering and sums).
    fn index_nodes(&mut self, tree: &[Vec<usize>], root: usize) {
        struct Frame {
            pre: usize,
            next: usize,
            postorder: isize,
            current_size: usize,
            desc_sizes: f64,
            kr_sizes_sum: f64,
            revkr_sizes_sum: f64,
        }
        let frame = |pre, postorder| Frame {
            pre,
            next: 0,
            postorder,
            current_size: 0,
            desc_sizes: 0.0,
            kr_sizes_sum: 0.0,
            revkr_sizes_sum: 0.0,
        };
        let p = self.add(root);
        let mut stack = vec![frame(p, -1)];
        // returned (child pre, desc_sizes_tmp) of the frame that finished last
        while let Some(top) = stack.last_mut() {
            let node = self.node[top.pre];
            if top.next < tree[node].len() {
                let child = tree[node][top.next];
                top.next += 1;
                let postorder = top.postorder;
                let cp = self.add(child);
                stack.push(frame(cp, postorder));
                continue;
            }
            let f = stack.pop().unwrap();
            let postorder = f.postorder + 1;
            self.post_ltr_info.push(f.pre);
            self.post_ltr[f.pre] = postorder;
            let nsize = f.current_size + 1;
            self.size[f.pre] = nsize;
            let desc_sizes_tmp = f.desc_sizes + nsize as f64;
            let ns = nsize as f64;
            self.desc_sum[f.pre] = (ns * (ns + 3.0)) / 2.0 - desc_sizes_tmp;
            self.kr_sum[f.pre] = f.kr_sizes_sum + ns;
            self.rev_kr_sum[f.pre] = f.revkr_sizes_sum + ns;
            if let Some(parent) = stack.last_mut() {
                let child = f.pre;
                self.parent[child] = parent.pre as isize;
                self.children[parent.pre].push(child);
                parent.postorder = postorder;
                parent.current_size += nsize;
                parent.desc_sizes += desc_sizes_tmp;
                parent.kr_sizes_sum += self.kr_sum[child];
                parent.revkr_sizes_sum += self.rev_kr_sum[child];
                let child_index = parent.next - 1;
                let last_index = tree[self.node[parent.pre]].len() - 1;
                if child_index == 0 {
                    parent.kr_sizes_sum -= nsize as f64;
                    self.type_l[child] = true;
                }
                if child_index == last_index {
                    parent.revkr_sizes_sum -= nsize as f64;
                    self.type_r[child] = true;
                }
            }
        }
    }

    fn post_traversal_indexing(&mut self) {
        let n = self.tree_size;
        self.pre_rtl_info = vec![0; n];
        self.post_rtl_info = vec![0; n];
        let mut current_leaf: isize = -1;
        for i in 0..n {
            self.pre_rtl[i] = n as isize - 1 - self.post_ltr[i];
            self.post_rtl[i] = n as isize - 1 - i as isize;
            self.pre_rtl_info[self.pre_rtl[i] as usize] = i;
            self.post_rtl_info[self.post_rtl[i] as usize] = i;
            self.lnl[i] = current_leaf;
            if self.children[i].is_empty() {
                current_leaf = i as isize;
                let parent = self.parent[i];
                if parent >= 0 {
                    let parent = parent as usize;
                    if parent + 1 == i {
                        self.lchl += 1;
                    } else if self.pre_rtl[parent] + 1 == self.pre_rtl[i] {
                        self.rchl += 1;
                    }
                }
            }
            let sum_node = n - i - 1;
            self.sum_cost[sum_node] += 1.0;
            let sum_parent = self.parent[sum_node];
            if sum_parent >= 0 {
                let v = self.sum_cost[sum_node];
                self.sum_cost[sum_parent as usize] += v;
            }
        }
        let mut current_leaf: isize = -1;
        for i in 0..n {
            let node = self.pre_rtl_info[i];
            self.lnr[node] = current_leaf;
            if self.children[node].is_empty() {
                current_leaf = node as isize;
            }
            let node = self.post_ltr_info[i];
            self.lld[node] = if self.size[node] == 1 {
                node
            } else {
                self.lld[self.children[node][0]]
            };
            let node = self.post_rtl_info[i];
            self.rld[node] = if self.size[node] == 1 {
                node
            } else {
                self.rld[*self.children[node].last().unwrap()]
            };
        }
    }

    // Accessors that accept the dummy node (-1).
    fn pre_ltr_of(&self, r: isize) -> isize {
        r
    }
    fn pre_rtl_of(&self, r: isize) -> isize {
        if r < 0 {
            -1
        } else {
            self.pre_rtl[r as usize]
        }
    }
    fn size_of(&self, r: isize) -> usize {
        if r < 0 {
            1
        } else {
            self.size[r as usize]
        }
    }
    fn sum_cost_of(&self, r: isize) -> f64 {
        if r < 0 {
            0.0
        } else {
            self.sum_cost[r as usize]
        }
    }
    fn leftmost(&self, r: isize) -> isize {
        if r < 0 {
            return -1;
        }
        self.children[r as usize]
            .first()
            .map_or(-1, |&c| c as isize)
    }
    fn rightmost(&self, r: isize) -> isize {
        if r < 0 {
            return -1;
        }
        self.children[r as usize].last().map_or(-1, |&c| c as isize)
    }
}

/// Python list indexing: negative indices count from the end.
#[inline]
fn wrap(i: isize, len: usize, err: &Cell<bool>) -> usize {
    let j = if i < 0 { i + len as isize } else { i };
    if j < 0 || j as usize >= len {
        err.set(true);
        0
    } else {
        j as usize
    }
}

const LEFT: u8 = 0;
const RIGHT: u8 = 1;
const INNER: u8 = 2;

#[derive(Clone, Copy, Default)]
struct Cost {
    l: f64,
    r: f64,
    i: f64,
    path: f64,
}

struct Apted<'a, R: FnMut(usize, usize) -> f64> {
    it1: &'a Indexer,
    it2: &'a Indexer,
    rename: R,
    /// `delta[pre_ltr1 * n2 + pre_ltr2]`
    delta: Vec<f64>,
    n2: usize,
    ren_cache: Vec<f64>,
    err: Cell<bool>,
}

impl<'a, R: FnMut(usize, usize) -> f64> Apted<'a, R> {
    fn d(&self, a: isize, b: isize) -> usize {
        let a = wrap(a, self.it1.tree_size, &self.err);
        let b = wrap(b, self.n2, &self.err);
        a * self.n2 + b
    }

    /// `config.rename(node1, node2)` for tree-1 / tree-2 preorder ids.
    fn ren(&mut self, p1: usize, p2: usize) -> f64 {
        // apted calls rename for the same pair many times; costs are pure, so cache them.
        let k = p1 * self.n2 + p2;
        let v = self.ren_cache[k];
        if !v.is_nan() {
            return v;
        }
        let (a, b) = (self.it1.node[p1], self.it2.node[p2]);
        let v = (self.rename)(a, b);
        self.ren_cache[k] = v;
        v
    }

    fn compute_opt_strategy(&mut self, left: bool) {
        let it1 = self.it1;
        let it2 = self.it2;
        let size1 = it1.tree_size;
        let size2 = it2.tree_size;
        let n2 = self.n2;
        let (order1, order2) = if left {
            (&it1.post_ltr_info, &it2.post_ltr_info)
        } else {
            (&it1.post_rtl_info, &it2.post_rtl_info)
        };
        let costi1 = |p: usize| {
            if left {
                it1.post_ltr[p] as usize
            } else {
                it1.post_rtl[p] as usize
            }
        };
        let costi2 = |p: usize| {
            if left {
                it2.post_ltr[p] as usize
            } else {
                it2.post_rtl[p] as usize
            }
        };
        let mut strategy = vec![0f64; size1 * n2];
        // rows[0] is the shared leaf row
        let mut rows: Vec<Vec<Cost>> = vec![vec![Cost::default(); size2]];
        let mut cost1: Vec<Option<usize>> = vec![None; size1];
        let path_id_offset = size1 as f64;
        let mut rows_to_reuse: Vec<usize> = Vec::new();

        let update_parent = |min_cost: f64,
                             type_l: bool,
                             type_r: bool,
                             cost: Cost,
                             parent: &mut Cost|
         -> Option<f64> {
            let mut update_path = None;
            if left {
                parent.r += min_cost;
                let tmp = -min_cost + cost.i;
                if tmp < parent.i {
                    parent.i = tmp;
                    parent.path = cost.path;
                    update_path = Some(cost.path);
                }
                if type_r {
                    parent.i += parent.r;
                    parent.r += cost.r - min_cost;
                }
                if type_l {
                    parent.l += cost.l;
                } else {
                    parent.l += min_cost;
                }
            } else {
                parent.l += min_cost;
                let tmp = -min_cost + cost.i;
                if tmp < parent.i {
                    parent.i = tmp;
                    parent.path = cost.path;
                    update_path = Some(cost.path);
                }
                if type_l {
                    parent.i += parent.l;
                    parent.l += cost.l - min_cost;
                }
                if type_r {
                    parent.r += cost.r;
                } else {
                    parent.r += min_cost;
                }
            }
            update_path
        };

        for &node1 in order1 {
            let v_cost = costi1(node1);
            let v_in_pre_ltr = node1;
            let size_v = it1.size[node1];
            let kr_sum_v = it1.kr_sum[node1];
            let rev_kr_sum_v = it1.rev_kr_sum[node1];
            let desc_sum_v = it1.desc_sum[node1];
            let left_path_v =
                -((it1.pre_rtl_info[(it1.pre_rtl[node1] as usize) + size_v - 1] as f64) + 1.0);
            let right_path_v = (v_in_pre_ltr + size_v) as f64;
            let row_v;
            if it1.children[node1].is_empty() {
                row_v = 0;
                cost1[v_cost] = Some(0);
                for &node2 in order2 {
                    let w_cost = costi2(node2);
                    rows[0][w_cost].path = v_in_pre_ltr as f64;
                    strategy[v_in_pre_ltr * n2 + node2] = v_in_pre_ltr as f64;
                }
            } else {
                row_v = cost1[v_cost].expect("child processed before parent");
            }
            let parent1 = it1.parent[node1];
            let mut row_parent = usize::MAX;
            if parent1 >= 0 {
                let parent_v = costi1(parent1 as usize);
                if cost1[parent_v].is_none() {
                    let r = match rows_to_reuse.pop() {
                        Some(r) => r,
                        None => {
                            rows.push(vec![Cost::default(); size2]);
                            rows.len() - 1
                        }
                    };
                    cost1[parent_v] = Some(r);
                }
                row_parent = cost1[parent_v].unwrap();
            }
            let mut cost2 = vec![Cost::default(); size2];
            for &node2 in order2 {
                let w_cost = costi2(node2);
                let w_in_pre_ltr = node2;
                let size_w = it2.size[node2];
                if it2.children[node2].is_empty() {
                    let c = &mut cost2[w_cost];
                    c.l = 0.0;
                    c.r = 0.0;
                    c.i = 0.0;
                    c.path = w_in_pre_ltr as f64;
                }
                let cpvw = rows[row_v][w_cost];
                let cpw = cost2[w_cost];
                let min_cost;
                let strategy_path;
                if size_v <= 1 || size_w <= 1 {
                    min_cost = size_v.max(size_w) as f64;
                    strategy_path = -1.0;
                } else {
                    let sv = size_v as f64;
                    let sw = size_w as f64;
                    let cands = [
                        sv * it2.kr_sum[node2] + cpvw.l,
                        sv * it2.rev_kr_sum[node2] + cpvw.r,
                        sv * it2.desc_sum[node2] + cpvw.i,
                        sw * kr_sum_v + cpw.l,
                        sw * rev_kr_sum_v + cpw.r,
                        sw * desc_sum_v + cpw.i,
                    ];
                    let mut k = 0;
                    for (m, &c) in cands.iter().enumerate() {
                        if c < cands[k] {
                            k = m;
                        }
                    }
                    min_cost = cands[k];
                    strategy_path = match k {
                        0 => left_path_v,
                        1 => right_path_v,
                        2 => cpvw.path + 1.0,
                        3 => {
                            -(it2.pre_rtl_info[(it2.pre_rtl[node2] as usize) + size_w - 1] as f64)
                                - path_id_offset
                                - 1.0
                        }
                        4 => (w_in_pre_ltr + size_w) as f64 + path_id_offset,
                        _ => cpw.path + path_id_offset + 1.0,
                    };
                }
                if parent1 >= 0 {
                    let mut pc = rows[row_parent][w_cost];
                    let new_path = update_parent(
                        min_cost,
                        it1.type_l[node1],
                        it1.type_r[node1],
                        rows[row_v][w_cost],
                        &mut pc,
                    );
                    rows[row_parent][w_cost] = pc;
                    if let Some(p) = new_path {
                        strategy[v_in_pre_ltr * n2 + w_in_pre_ltr] = p;
                    }
                }
                let parent2 = it2.parent[node2];
                if parent2 >= 0 {
                    let pw = costi2(parent2 as usize);
                    let mut pc = cost2[pw];
                    update_parent(
                        min_cost,
                        it2.type_l[node2],
                        it2.type_r[node2],
                        cost2[w_cost],
                        &mut pc,
                    );
                    cost2[pw] = pc;
                }
                rows[row_v][w_cost].path = strategy_path;
                strategy[v_in_pre_ltr * n2 + w_in_pre_ltr] = strategy_path;
            }
            if !it1.children[node1].is_empty() {
                for c in rows[row_v].iter_mut() {
                    c.l = 0.0;
                    c.r = 0.0;
                    c.i = 0.0;
                }
                rows_to_reuse.push(row_v);
            }
        }
        self.delta = strategy;
    }

    fn ted_init(&mut self) {
        let (it1, it2) = (self.it1, self.it2);
        for x in 0..it1.tree_size {
            let size1 = it1.size[x];
            for y in 0..it2.tree_size {
                let size2 = it2.size[y];
                let v = &mut self.delta[x * self.n2 + y];
                if size1 == 1 && size2 == 1 {
                    *v = 0.0;
                } else if size1 == 1 {
                    *v = it2.sum_cost[y] - 1.0;
                } else if size2 == 1 {
                    *v = it1.sum_cost[x] - 1.0;
                }
            }
        }
    }

    /// `spf1` (unit insert/delete).
    fn spf1(&mut self, t1: usize, t2: usize) -> f64 {
        let (it1, it2) = (self.it1, self.it2);
        if it1.size[t1] == 1 && it2.size[t2] == 1 {
            let r = self.ren(t1, t2);
            return if r < 2.0 { r } else { 2.0 };
        }
        if it1.size[t1] == 1 {
            let cost = it2.sum_cost[t2];
            let max_cost = cost + 1.0;
            let mut m = cost;
            for p in t2..t2 + it2.size[t2] {
                let v = self.ren(t1, p) - 1.0;
                if v < m {
                    m = v;
                }
            }
            let a = m + cost;
            return if max_cost < a { max_cost } else { a };
        }
        let cost = it1.sum_cost[t1];
        let max_cost = cost + 1.0;
        let mut m = cost;
        for p in t1..t1 + it1.size[t1] {
            let v = self.ren(p, t2) - 1.0;
            if v < m {
                m = v;
            }
        }
        let a = m + cost;
        if max_cost < a {
            max_cost
        } else {
            a
        }
    }

    fn gted(&mut self, t1: usize, t2: usize) -> f64 {
        if self.err.get() {
            return 0.0;
        }
        let (it1, it2) = (self.it1, self.it2);
        if it1.size[t1] == 1 || it2.size[t2] == 1 {
            let r = self.spf1(t1, t2);
            self.delta[t1 * self.n2 + t2] = r;
            return r;
        }
        let path_id = self.delta[t1 * self.n2 + t2].trunc() as isize;
        let node_id = path_id.abs() - 1;
        if node_id < it1.tree_size as isize {
            return self.sub_gted(t1, t2, path_id, node_id, false);
        }
        let node_id = node_id - it1.tree_size as isize;
        self.sub_gted(t1, t2, path_id, node_id, true)
    }

    /// `sub_gted`; `reverse` means the path is in tree 2 (`it_f` = tree 2). `node_id` may be
    /// -1 (a distance stored in `delta` read as a path id), which Python wraps.
    fn sub_gted(
        &mut self,
        t1: usize,
        t2: usize,
        path_id: isize,
        node_id: isize,
        reverse: bool,
    ) -> f64 {
        let (it_f, tree_f) = if reverse {
            (self.it2, t2)
        } else {
            (self.it1, t1)
        };
        let size1 = self.it1.tree_size as isize;
        // get_strategy_path_type
        let strategy = if path_id < 0 {
            LEFT
        } else {
            let mut p = path_id.abs() - 1;
            if p >= size1 {
                p -= size1;
            }
            if p == (tree_f + it_f.size[tree_f] - 1) as isize {
                RIGHT
            } else {
                INNER
            }
        };
        // traverse_up(current, tree_f)
        let mut current = wrap(node_id, it_f.tree_size, &self.err);
        let mut parent = it_f.parent[current];
        while parent >= 0 && parent as usize >= tree_f {
            let p = parent as usize;
            for &child in &it_f.children[p] {
                if child != current {
                    if reverse {
                        self.gted(t1, child);
                    } else {
                        self.gted(child, t2);
                    }
                }
            }
            current = p;
            parent = it_f.parent[p];
        }
        if self.err.get() {
            return 0.0;
        }
        let (c1, c2) = if reverse { (t2, t1) } else { (t1, t2) };
        let mut spf = Spf::new(self, c1, c2, node_id, reverse);
        match strategy {
            LEFT => spf.spf_side(true),
            RIGHT => spf.spf_side(false),
            _ => spf.loop_a(strategy),
        }
    }
}

/// The two orientations of spfA's loops (`atb`/`bta`/... lambdas of `loop_a`).
#[derive(Clone, Copy, PartialEq)]
enum Dir {
    /// atb = pre_ltr, bta = pre_rtl, amost = leftmost, bmost = rightmost, lna = lnl
    Ltr,
    /// atb = pre_rtl, bta = pre_ltr, amost = rightmost, bmost = leftmost, lna = lnr
    Rtl,
}

/// `SinglePathFunction`: `f` is `it1` of the Python object (the tree holding the path),
/// `s` its `it2`; `swapped` when `f` is the APTED tree 2.
struct Spf<'s, 'a, R: FnMut(usize, usize) -> f64> {
    ap: &'s mut Apted<'a, R>,
    f: &'a Indexer,
    s_it: &'a Indexer,
    swapped: bool,
    current1: usize,
    current2: usize,
    path_id: isize,
    it2_loff: isize,
    it2_roff: isize,
    max_size: usize,
    fnv: Vec<isize>,
    ft: Vec<isize>,
    q: Vec<f64>,
    s: Vec<f64>,
    s_cols: usize,
    s_rows: usize,
    t: Vec<f64>,
    t_n: usize,
    v: isize,
    vparent: usize,
    it1_off: isize,
    dir: Dir,
    it2_doff: isize,
    a_part: bool,
    cost1: f64,
    size1: usize,
    cost2: f64,
}

impl<'s, 'a, R: FnMut(usize, usize) -> f64> Spf<'s, 'a, R> {
    fn new(ap: &'s mut Apted<'a, R>, c1: usize, c2: usize, path_id: isize, swapped: bool) -> Self {
        let (f, s_it) = if swapped {
            (ap.it2, ap.it1)
        } else {
            (ap.it1, ap.it2)
        };
        let max_size = f.tree_size.max(s_it.tree_size) + 1;
        let size1 = f.size[c1] + 1;
        let size2 = s_it.size[c2] + 1;
        Spf {
            f,
            s_it,
            swapped,
            current1: c1,
            current2: c2,
            path_id,
            it2_loff: c2 as isize,
            it2_roff: s_it.pre_rtl[c2],
            max_size,
            fnv: vec![0; max_size + 1],
            ft: vec![0; max_size + 1],
            q: vec![0.0; max_size],
            s: vec![0.0; size1 * size2],
            s_cols: size2,
            s_rows: size1,
            t: vec![0.0; size2 * size2],
            t_n: size2,
            v: -1,
            vparent: 0,
            it1_off: -1,
            dir: Dir::Ltr,
            it2_doff: -1,
            a_part: false,
            cost1: 0.0,
            size1: 0,
            cost2: 0.0,
            ap,
        }
    }

    fn err(&self) -> &Cell<bool> {
        &self.ap.err
    }

    /// `rename` of the single-path function: arguments in (f, s) order.
    fn rename(&mut self, pf: usize, ps: usize) -> f64 {
        if self.swapped {
            self.ap.ren(ps, pf)
        } else {
            self.ap.ren(pf, ps)
        }
    }

    /// `delta[a][b]` with `a, b = swap(x, y)`.
    fn delta_idx(&self, x: isize, y: isize) -> usize {
        if self.swapped {
            self.ap.d(y, x)
        } else {
            self.ap.d(x, y)
        }
    }

    fn s_idx(&self, r: isize, c: isize) -> usize {
        wrap(r, self.s_rows, self.err()) * self.s_cols + wrap(c, self.s_cols, self.err())
    }

    fn t_idx(&self, r: isize, c: isize) -> usize {
        wrap(r, self.t_n, self.err()) * self.t_n + wrap(c, self.t_n, self.err())
    }

    /// `get(t, ag, bg, left_right, -loff2, -roff2)`
    fn t_get(&self, ag: isize, bg: isize) -> f64 {
        let (lg, rg) = self.lr(ag, bg);
        self.t[self.t_idx(lg - self.it2_loff, rg - self.it2_roff)]
    }

    fn lr(&self, ag: isize, bg: isize) -> (isize, isize) {
        match self.dir {
            Dir::Ltr => (ag, bg),
            Dir::Rtl => (bg, ag),
        }
    }

    // ---- spf_l / spf_r ----

    fn spf_side(&mut self, left: bool) -> f64 {
        let (current1, current2) = (self.current1, self.current2);
        let size1 = self.f.size[current1];
        let size2 = self.s_it.size[current2];
        let s_it = self.s_it;
        let ald = |p: usize| if left { s_it.lld[p] } else { s_it.rld[p] };
        let mut keyroots = Vec::with_capacity(size2);
        // compute_keyroots (recursive in Python; same order with an explicit stack)
        fn keyroots_rec(s_it: &Indexer, left: bool, root: usize, keyroots: &mut Vec<usize>) {
            keyroots.push(root);
            let ald = |p: usize| if left { s_it.lld[p] } else { s_it.rld[p] };
            let mut path = ald(root);
            while path > root {
                let parent = s_it.parent[path] as usize;
                for &child in &s_it.children[parent] {
                    if child != path {
                        keyroots_rec(s_it, left, child, keyroots);
                    }
                }
                path = parent;
            }
        }
        let _ = ald;
        keyroots_rec(s_it, left, current2, &mut keyroots);
        let cols = size2 + 1;
        let mut forestdist = vec![0f64; (size1 + 1) * cols];
        for i in (0..keyroots.len()).rev() {
            self.tree_edit_dist(left, current1, keyroots[i], &mut forestdist, cols);
            if self.err().get() {
                return 0.0;
            }
        }
        forestdist[size1 * cols + size2]
    }

    fn tree_edit_dist(
        &mut self,
        left: bool,
        sub1: usize,
        sub2: usize,
        fd: &mut [f64],
        cols: usize,
    ) {
        let (f, s_it) = (self.f, self.s_it);
        let atb1 = |p: usize| if left { f.post_ltr[p] } else { f.post_rtl[p] };
        let atb2 = |p: usize| {
            if left {
                s_it.post_ltr[p]
            } else {
                s_it.post_rtl[p]
            }
        };
        let ald1 = |p: usize| if left { f.lld[p] } else { f.rld[p] };
        let ald2 = |p: usize| if left { s_it.lld[p] } else { s_it.rld[p] };
        let info1 = |k: isize| {
            if left {
                f.post_ltr_info[k as usize]
            } else {
                f.post_rtl_info[k as usize]
            }
        };
        let info2 = |k: isize| {
            if left {
                s_it.post_ltr_info[k as usize]
            } else {
                s_it.post_rtl_info[k as usize]
            }
        };
        let i = atb1(sub1);
        let j = atb2(sub2);
        let ioff = atb1(ald1(sub1)) - 1;
        let joff = atb2(ald2(sub2)) - 1;
        let rows = (i - ioff) as usize;
        let ncols = (j - joff) as usize;
        fd[0] = 0.0;
        for i1 in 1..=rows {
            fd[i1 * cols] = fd[(i1 - 1) * cols] + 1.0;
        }
        for j1 in 1..=ncols {
            fd[j1] = fd[j1 - 1] + 1.0;
        }
        let ald_sub1 = ald1(sub1);
        let ald_sub2 = ald2(sub2);
        for i1 in 1..=rows {
            let p1 = info1(i1 as isize + ioff);
            let p1_ald = ald1(p1);
            for j1 in 1..=ncols {
                let p2 = info2(j1 as isize + joff);
                let p2_ald = ald2(p2);
                let u = self.rename(p1, p2);
                let da = fd[(i1 - 1) * cols + j1] + 1.0;
                let db = fd[i1 * cols + j1 - 1] + 1.0;
                let di = self.delta_idx(p1 as isize, p2 as isize);
                let dc;
                if p1_ald == ald_sub1 && p2_ald == ald_sub2 {
                    dc = fd[(i1 - 1) * cols + j1 - 1] + u;
                    self.ap.delta[di] = fd[(i1 - 1) * cols + j1 - 1];
                } else {
                    let id1 = (atb1(p1_ald) - 1 - ioff) as usize;
                    let id2 = (atb2(p2_ald) - 1 - joff) as usize;
                    dc = fd[id1 * cols + id2] + self.ap.delta[di] + u;
                }
                fd[i1 * cols + j1] = min3(da, db, dc);
            }
        }
    }

    // ---- spf_a ----

    fn atb_s(&self, r: isize) -> isize {
        match self.dir {
            Dir::Ltr => self.s_it.pre_ltr_of(r),
            Dir::Rtl => self.s_it.pre_rtl_of(r),
        }
    }
    fn bta_s(&self, r: isize) -> isize {
        match self.dir {
            Dir::Ltr => self.s_it.pre_rtl_of(r),
            Dir::Rtl => self.s_it.pre_ltr_of(r),
        }
    }
    fn atb_f(&self, r: isize) -> isize {
        match self.dir {
            Dir::Ltr => self.f.pre_ltr_of(r),
            Dir::Rtl => self.f.pre_rtl_of(r),
        }
    }
    fn bta_f(&self, r: isize) -> isize {
        match self.dir {
            Dir::Ltr => self.f.pre_rtl_of(r),
            Dir::Rtl => self.f.pre_ltr_of(r),
        }
    }
    /// `atb_1[k]` (f tree info list in atb order)
    fn atb_1(&self, k: isize) -> isize {
        let n = self.f.tree_size;
        let k = wrap(k, n, self.err());
        match self.dir {
            Dir::Ltr => k as isize,
            Dir::Rtl => self.f.pre_rtl_info[k] as isize,
        }
    }
    fn atb_2(&self, k: isize) -> isize {
        let n = self.s_it.tree_size;
        let k = wrap(k, n, self.err());
        match self.dir {
            Dir::Ltr => k as isize,
            Dir::Rtl => self.s_it.pre_rtl_info[k] as isize,
        }
    }
    fn bta_2(&self, k: isize) -> isize {
        let n = self.s_it.tree_size;
        let k = wrap(k, n, self.err());
        match self.dir {
            Dir::Ltr => self.s_it.pre_rtl_info[k] as isize,
            Dir::Rtl => k as isize,
        }
    }
    fn amost_s(&self, r: isize) -> isize {
        match self.dir {
            Dir::Ltr => self.s_it.leftmost(r),
            Dir::Rtl => self.s_it.rightmost(r),
        }
    }
    fn bmost_s(&self, r: isize) -> isize {
        match self.dir {
            Dir::Ltr => self.s_it.rightmost(r),
            Dir::Rtl => self.s_it.leftmost(r),
        }
    }
    fn lna_s(&self, r: isize) -> isize {
        if r < 0 {
            return -1; // EMPTY.lnl is EMPTY
        }
        match self.dir {
            Dir::Ltr => self.s_it.lnl[r as usize],
            Dir::Rtl => self.s_it.lnr[r as usize],
        }
    }

    fn loop_a(&mut self, path: u8) -> f64 {
        let f = self.f;
        let path_leaf = wrap(self.path_id, f.tree_size, self.err());
        let current1 = self.current1;
        let mut result = f64::NAN; // Python's initial tuple: never returned for valid input
                                   // walk_up(path_leaf, current1): (path_leaf, fake child) then traverse_up
        let mut steps: Vec<(usize, isize)> = vec![(path_leaf, -1)];
        let mut current = path_leaf;
        let mut parent = f.parent[current];
        while parent >= 0 && parent as usize >= current1 {
            steps.push((parent as usize, current as isize));
            current = parent as usize;
            parent = f.parent[current];
        }
        for (vparent, v) in steps {
            self.vparent = vparent;
            self.v = v;
            let vp_ltr = vparent as isize;
            let vp_rtl = f.pre_rtl[vparent];
            let v_ltr = v;
            let v_rtl = f.pre_rtl_of(v);
            let vt = v >= 0;
            let left_part = vt && v_ltr - vp_ltr > 1;
            let right_part = vt && v_rtl - vp_rtl > 1;
            let neither = !left_part && !right_part;
            if path == RIGHT || (path == INNER && left_part) {
                let (rf_first, lf_first) = if vt {
                    (v_rtl, v_ltr - 1)
                } else {
                    (vp_rtl, vp_ltr)
                };
                let lf_last = vp_ltr + right_part as isize;
                let rf_last = if right_part { -1 } else { vp_rtl };
                let lf_range: Vec<isize> = range_desc(lf_first, lf_last);
                self.it1_off = vp_ltr;
                self.it2_doff = self.it2_loff;
                self.dir = Dir::Ltr;
                self.a_part = left_part;
                result = self.loop_b(&lf_range, rf_first, rf_last);
            }
            if path == LEFT || (path == INNER && (right_part || neither)) {
                let (lf_first, rf_first) = if vt {
                    (vp_ltr + 1, v_rtl - 1)
                } else {
                    (vp_ltr, vp_rtl)
                };
                let rf_last = vp_rtl;
                let lf_last = vp_ltr;
                let rf_range: Vec<isize> = range_desc(rf_first, rf_last);
                self.it1_off = vp_rtl;
                self.it2_doff = self.it2_roff;
                self.dir = Dir::Rtl;
                self.a_part = right_part;
                result = self.loop_b(&rf_range, lf_first, lf_last);
            }
            if self.err().get() {
                return 0.0;
            }
        }
        if result.is_nan() {
            self.err().set(true);
        }
        result
    }

    fn loop_b(&mut self, af_range: &[isize], bf_first: isize, bf_last: isize) -> f64 {
        let mut result = f64::NAN;
        self.fnv = vec![-1; self.max_size];
        self.ft = vec![-1; self.max_size];
        let (tmpsize1, tmpcost1) = (self.size1, self.cost1);
        let current2 = self.current2 as isize;
        let current2_atb = self.atb_s(current2);
        let bg_last = self.bta_s(current2);
        let bg_first = bg_last + self.s_it.size[self.current2] as isize - 1;
        let mut bg = bg_first;
        while bg >= bg_last {
            let bg_info = self.bta_2(bg);
            let bg_atb = self.atb_s(bg_info);
            let bg_ln = self.atb_s(self.lna_s(bg_info));
            self.update_fn_array(bg_ln, bg_atb, current2_atb);
            self.update_ft_array(bg_ln, bg_atb);
            let ag_last = if bg_info == current2 {
                current2
            } else {
                self.amost_s(current2)
            };
            let ag_range = self.each_ft(bg_info, ag_last);
            self.size1 = tmpsize1;
            self.cost1 = tmpcost1;
            result = self.loop_c(
                bg,
                af_range,
                &ag_range,
                bf_first,
                bf_last,
                self.s_it.size_of(bg_info),
            );
            let bg_parent = if bg_info >= 0 {
                self.s_it.parent[bg_info as usize]
            } else {
                -1
            };
            let is_bmost = bg > bg_last && bg_parent >= 0 && bg_info == self.bmost_s(bg_parent);
            self.update_delta(af_range, &ag_range, bg, bg_parent, is_bmost);
            if self.err().get() {
                return 0.0;
            }
            bg -= 1;
        }
        result
    }

    fn update_fn_array(&mut self, ln_index: isize, node_index: isize, current_index: isize) {
        let ln = if ln_index >= current_index {
            ln_index
        } else {
            -1
        };
        let n = self.fnv.len();
        let ln_i = wrap(ln, n, self.err());
        let node_i = wrap(node_index, n, self.err());
        let old = self.fnv[ln_i];
        self.fnv[node_i] = old;
        self.fnv[ln_i] = node_index;
    }

    fn update_ft_array(&mut self, ln_index: isize, node_index: isize) {
        let n = self.ft.len();
        let node_i = wrap(node_index, n, self.err());
        self.ft[node_i] = ln_index;
        let fnn = self.fnv[wrap(node_index, self.fnv.len(), self.err())];
        if fnn > -1 {
            let k = wrap(fnn, n, self.err());
            self.ft[k] = node_index;
        }
    }

    /// `list(each_ft(start, finish))` as (atb index, node)
    fn each_ft(&self, start: isize, finish: isize) -> Vec<(isize, isize)> {
        let finish_atb = self.atb_s(finish);
        let mut out = Vec::new();
        let mut cur_atb = self.atb_s(start);
        out.push((cur_atb, start));
        cur_atb = self.ft[wrap(cur_atb, self.ft.len(), self.err())];
        while cur_atb >= finish_atb {
            let cur = self.atb_2(cur_atb);
            out.push((cur_atb, cur));
            cur_atb = self.ft[wrap(cur_atb, self.ft.len(), self.err())];
            if self.err().get() || out.len() > self.ft.len() + 1 {
                self.err().set(true);
                break;
            }
        }
        out
    }

    fn update_delta(
        &mut self,
        af_range: &[isize],
        ag_range: &[(isize, isize)],
        bg: isize,
        bg_parent: isize,
        is_bmost: bool,
    ) {
        let (loff2, roff2, doff2, off1) =
            (self.it2_loff, self.it2_roff, self.it2_doff, self.it1_off);
        let vp = self.vparent;
        let vp_parent = self.f.parent[vp];
        let Some(&af_last) = af_range.last() else {
            self.err().set(true);
            return;
        };
        if is_bmost {
            let amost = self.atb_s(bg_parent) + 1;
            if self.a_part {
                let di = self.delta_idx(vp as isize, bg_parent);
                let v = self.s[self.s_idx(af_last + 1 - off1, amost - doff2)];
                self.ap.delta[di] = v;
            }
            if vp_parent >= 0
                && vp as isize == self.f.rightmost(vp_parent)
                && self.f.rightmost(vp_parent) == self.f.leftmost(vp_parent)
            {
                let di = self.delta_idx(vp_parent, bg_parent);
                let v = self.s[self.s_idx(af_last - off1, amost - doff2)];
                self.ap.delta[di] = v;
            }
            for &af in af_range {
                let v = self.s[self.s_idx(af - off1, amost - doff2)];
                let qi = wrap(af, self.q.len(), self.err());
                self.q[qi] = v;
            }
        }
        for &(ag, _) in ag_range {
            let (lg, rg) = self.lr(ag, bg);
            let v = self.s[self.s_idx(af_last - off1, ag - doff2)];
            let ti = self.t_idx(lg - loff2, rg - roff2);
            self.t[ti] = v;
        }
    }

    fn loop_c(
        &mut self,
        bg: isize,
        af_range: &[isize],
        ag_range: &[(isize, isize)],
        mut bf: isize,
        bf_last: isize,
        g_size: usize,
    ) -> f64 {
        let off1 = self.it1_off;
        let doff2 = self.it2_doff;
        let v_atb = self.atb_f(self.v);
        let mut result = f64::NAN;
        let Some(&af_last) = af_range.last() else {
            self.err().set(true);
            return 0.0;
        };
        for &af in af_range {
            let af_info = self.atb_1(af);
            if af == af_last {
                bf = bf_last;
            }
            self.size1 += 1;
            self.cost1 += 1.0;
            self.cost2 = 0.0;
            let af_subtree_size = self.f.size_of(af_info) as isize;
            let distance = v_atb - af;
            let consecutive = distance == 1;
            let side_sibling = distance == af_subtree_size;
            let mut sp1source = 1;
            let mut sp3source = 1;
            if consecutive {
                sp1source = 2;
            }
            let sp3_0;
            if self.bta_f(af_info) == bf {
                sp3_0 = 0.0;
                if af_subtree_size == 1 {
                    sp1source = 3;
                }
                sp3source = 2;
            } else {
                sp3_0 = self.cost1 - self.f.sum_cost_of(af_info);
                if side_sibling {
                    sp3source = 3;
                }
            }
            let ctx = LoopCtx {
                bg,
                af,
                af_pre: self.f.pre_ltr_of(af_info),
                g_size,
                sp1_row: af + 1 - off1,
                sp2_row: af - off1,
                sp3_row: if sp3source == 1 {
                    af + af_subtree_size - off1
                } else {
                    0
                },
                sp1source,
                sp3source,
                sp3_0,
                doff2,
            };
            if af_info < 0 {
                self.err().set(true);
                return 0.0;
            }
            result = self.loop_d(&ctx, ag_range, af_info as usize);
            if self.err().get() {
                return 0.0;
            }
        }
        result
    }

    fn loop_d(&mut self, c: &LoopCtx, ag_range: &[(isize, isize)], f_node: usize) -> f64 {
        let mut result = f64::NAN;
        for (k, &(ag, ag_info)) in ag_range.iter().enumerate() {
            if ag_info < 0 {
                self.err().set(true);
                return 0.0;
            }
            let after = k > 0;
            if after {
                self.cost2 += 1.0;
            } else {
                self.cost2 += self.s_it.sum_cost[ag_info as usize];
            }
            let ag_info = ag_info as usize;
            // calculate_min_loop_d
            let sp1 = match c.sp1source {
                1 => self.s[self.s_idx(c.sp1_row, ag - c.doff2)],
                2 => self.t_get(ag, c.bg),
                _ => self.cost2,
            } + 1.0;
            let sp2 = if after {
                let fa = self.fnv[wrap(ag, self.fnv.len(), self.err())];
                self.s[self.s_idx(c.sp2_row, fa - c.doff2)]
            } else if c.g_size == 1 {
                self.cost1
            } else {
                self.q[wrap(c.af, self.q.len(), self.err())]
            } + 1.0;
            let sp3_pre = self.ap.delta[self.delta_idx(c.af_pre, ag_info as isize)];
            let sp3 = if after {
                let size = self.s_it.size[ag_info] as isize;
                match c.sp3source {
                    1 => {
                        let fa = self.fnv[wrap(ag + size - 1, self.fnv.len(), self.err())];
                        self.s[self.s_idx(c.sp3_row, fa - c.doff2)]
                    }
                    2 => self.cost2 - self.s_it.sum_cost[ag_info],
                    _ => {
                        let fa = self.fnv[wrap(ag + size - 1, self.fnv.len(), self.err())];
                        self.t_get(fa, c.bg)
                    }
                }
            } else {
                c.sp3_0
            };
            let u = self.rename(f_node, ag_info);
            let v = min3(sp1, sp2, sp3_pre + sp3 + u);
            result = v;
            let wi = self.s_idx(c.sp2_row, ag - c.doff2);
            self.s[wi] = v;
        }
        result
    }
}

struct LoopCtx {
    bg: isize,
    af: isize,
    af_pre: isize,
    g_size: usize,
    sp1_row: isize,
    sp2_row: isize,
    sp3_row: isize,
    sp1source: u8,
    sp3source: u8,
    sp3_0: f64,
    doff2: isize,
}

/// `range(first, last - 1, -1)`
fn range_desc(first: isize, last: isize) -> Vec<isize> {
    let mut v = Vec::new();
    let mut x = first;
    while x >= last {
        v.push(x);
        x -= 1;
    }
    v
}

/// Python's `min(a, b, c)`.
#[inline]
fn min3(a: f64, b: f64, c: f64) -> f64 {
    let mut m = a;
    if b < m {
        m = b;
    }
    if c < m {
        m = c;
    }
    m
}

/// `APTED(tree1, tree2, config).compute_edit_distance()` of apted 1.0.3 with unit
/// insert/delete costs and `rename(node1, node2)` on node ids. Trees are child lists plus a
/// root id, as for [`crate::tree_edit_distance`]. `Err(Error::Apted)` where apted raises
/// `IndexError`.
pub fn apted<R: FnMut(usize, usize) -> f64>(
    children1: &[Vec<usize>],
    root1: usize,
    children2: &[Vec<usize>],
    root2: usize,
    rename: R,
) -> Result<f64, crate::Error> {
    let it1 = Indexer::new(children1, root1, 0);
    let it2 = Indexer::new(children2, root2, 1);
    debug_assert_eq!(it1.num, 0);
    let mut ap = Apted {
        it1: &it1,
        it2: &it2,
        rename,
        delta: Vec::new(),
        n2: it2.tree_size,
        ren_cache: vec![f64::NAN; it1.tree_size * it2.tree_size],
        err: Cell::new(false),
    };
    ap.compute_opt_strategy(it1.lchl < it1.rchl);
    ap.ted_init();
    let r = ap.gted(0, 0);
    if ap.err.get() {
        return Err(crate::Error::Apted);
    }
    Ok(r)
}
