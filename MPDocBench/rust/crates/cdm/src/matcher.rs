//! Port of `metrics/cdm/modules/visual_matcher.py` (HungarianMatcher, SimpleAffineTransform,
//! norm_same_token) and of `skimage.measure.ransac` 0.20.0 as CDM calls it.

use crate::lsap::linear_sum_assignment;
use crate::rng::Pcg64;

pub struct BoxTok {
    pub bbox: [i64; 4],
    pub token: String,
}

pub fn norm_same_token(token: &str) -> String {
    let mut token: String = match token {
        "\\cdot" => ".",
        "\\mid" => "|",
        "\\to" => "\\rightarrow",
        "\\top" => "T",
        "\\Tilde" => "\\tilde",
        "\\cdots" => "\\dots",
        "\\prime" => "'",
        "\\ast" => "*",
        "\\left<" => "\\langle",
        "\\right>" => "\\rangle",
        t => t,
    }
    .to_string();
    if token.starts_with("\\left") || token.starts_with("\\right") {
        token = token.replace("\\left", "").replace("\\right", "");
    }
    if token.starts_with("\\big") || token.starts_with("\\Big") {
        let tail = &token[4..];
        if tail.contains('\\') {
            token = format!("\\{}", tail.rsplit('\\').next().unwrap());
        } else {
            token = token.chars().last().unwrap().to_string();
        }
    }
    if token == "\\leq" || token == "\\geq" {
        return token[..token.len() - 1].to_string();
    }
    if ["\\lVert", "\\rVert", "\\Vert"].contains(&token.as_str()) {
        return "\\|".to_string();
    }
    if ["\\lvert", "\\rvert", "\\vert"].contains(&token.as_str()) {
        return "|".to_string();
    }
    if token.ends_with("rightarrow") {
        return "\\rightarrow".to_string();
    }
    if token.ends_with("leftarrow") {
        return "\\leftarrow".to_string();
    }
    if token.starts_with("\\wide") {
        return token.replace("wide", "");
    }
    if token.starts_with("\\var") {
        return token.replace("\\var", "");
    }
    token
}

pub struct Matched {
    pub pairs: Vec<(usize, usize)>,
    /// token cost, row-major n_gt x n_pred
    pub token_cost: Vec<f64>,
    pub n_pred: usize,
}

/// `HungarianMatcher()(box_gt, box_pred, gt_size, pred_size)`; None where numpy raises
/// (an empty box list makes the fancy indexing in calculate_token_cost fail).
pub fn hungarian_match(box_gt: &[BoxTok], box_pred: &[BoxTok], gt_size: (usize, usize), pred_size: (usize, usize)) -> Option<Matched> {
    let (ng, np_) = (box_gt.len(), box_pred.len());
    if ng == 0 || np_ == 0 {
        return None;
    }
    let to_arr = |b: &[BoxTok], (w, h): (usize, usize)| -> Vec<[f64; 4]> {
        let (w, h) = (w as f64, h as f64);
        b.iter()
            .map(|x| [x.bbox[0] as f64 / w, x.bbox[1] as f64 / h, x.bbox[2] as f64 / w, x.bbox[3] as f64 / h])
            .collect()
    };
    let ga = to_arr(box_gt, gt_size);
    let pa = to_arr(box_pred, pred_size);
    let norm_g: Vec<String> = box_gt.iter().map(|b| norm_same_token(&b.token)).collect();
    let norm_p: Vec<String> = box_pred.iter().map(|b| norm_same_token(&b.token)).collect();
    let mut token_cost = vec![0.0f64; ng * np_];
    let mut cost = vec![0.0f64; ng * np_];
    for i in 0..ng {
        for j in 0..np_ {
            let t = if box_gt[i].token == box_pred[j].token {
                0.0
            } else if norm_g[i] == norm_p[j] {
                0.05
            } else {
                1.0
            };
            token_cost[i * np_ + j] = t;
            // cdist(..., 'minkowski', p=1) / 4: |d0| + |d1| + |d2| + |d3| in order
            let mut l1 = 0.0f64;
            for k in 0..4 {
                l1 += (ga[i][k] - pa[j][k]).abs();
            }
            let position = l1 / 4.0;
            let order = (i as f64 / ng as f64 - j as f64 / np_ as f64).abs() / 1.0;
            let mut c = 1.0 * t + 0.05 * position + 0.15 * order;
            if c.is_nan() || c.is_infinite() {
                c = 100.0;
            }
            cost[i * np_ + j] = c;
        }
    }
    let (rows, cols) = linear_sum_assignment(&cost, ng, np_);
    Some(Matched { pairs: rows.into_iter().zip(cols).collect(), token_cost, n_pred: np_ })
}

/// `np.mean(a, axis=0)` of an (N, 2) array (sequential sums).
fn mean2(a: &[[f64; 2]]) -> [f64; 2] {
    let mut s = [0.0f64; 2];
    for p in a {
        s[0] += p[0];
        s[1] += p[1];
    }
    let n = a.len() as f64;
    [s[0] / n, s[1] / n]
}

/// 1-D `np.mean` of a short array (n < 9: sequential).
fn mean1(a: &[f64]) -> f64 {
    let mut s = 0.0;
    for &x in a {
        s += x;
    }
    s / a.len() as f64
}

struct Affine {
    translation: [f64; 2],
    scale: f64,
}

impl Affine {
    fn estimate(src: &[[f64; 2]], dst: &[[f64; 2]]) -> Affine {
        let sc = mean2(src);
        let dc = mean2(dst);
        let translation = [dc[0] - sc[0], dc[1] - sc[1]];
        let sd: Vec<f64> = src.iter().map(|p| ((p[0] - sc[0]) * (p[0] - sc[0]) + (p[1] - sc[1]) * (p[1] - sc[1])).sqrt()).collect();
        let dd: Vec<f64> = dst.iter().map(|p| ((p[0] - dc[0]) * (p[0] - dc[0]) + (p[1] - dc[1]) * (p[1] - dc[1])).sqrt()).collect();
        let scale = mean1(&dd) / (mean1(&sd) + 1e-10);
        Affine { translation, scale }
    }

    fn residuals(&self, src: &[[f64; 2]], dst: &[[f64; 2]]) -> Vec<f64> {
        let m = mean2(src);
        src.iter()
            .zip(dst)
            .map(|(p, d)| {
                let x = self.scale * (p[0] - m[0]) + m[0] + self.translation[0];
                let y = self.scale * (p[1] - m[1]) + m[1] + self.translation[1];
                let ex = x - d[0];
                let ey = y - d[1];
                (ex * ex + ey * ey).sqrt()
            })
            .collect()
    }
}

/// `v.dot(v)` as OpenBLAS 0.3.21 `ddot` (armv8 kernel `dot.S`) computes it: two lanes over
/// blocks of 4 (separate multiply and add), lanes summed, then fused multiply-add tail.
fn blas_dot_self(v: &[f64]) -> f64 {
    let n = v.len();
    let (mut a0, mut a1) = (0.0f64, 0.0f64);
    let mut i = 0;
    while i + 4 <= n {
        a0 += v[i] * v[i];
        a1 += v[i + 1] * v[i + 1];
        a0 += v[i + 2] * v[i + 2];
        a1 += v[i + 3] * v[i + 3];
        i += 4;
    }
    let mut s = a0 + a1;
    while i < n {
        s = v[i].mul_add(v[i], s);
        i += 1;
    }
    s
}

const EPSILON: f64 = f64::EPSILON; // np.spacing(1)

fn dynamic_max_trials(n_inliers: usize, n_samples: usize, min_samples: i32, probability: f64) -> f64 {
    if probability == 0.0 {
        return 0.0;
    }
    if n_inliers == 0 {
        return f64::INFINITY;
    }
    let inlier_ratio = n_inliers as f64 / n_samples as f64;
    let nom = EPSILON.max(1.0 - probability);
    let denom = EPSILON.max(1.0 - inlier_ratio.powf(min_samples as f64));
    (nom.ln() / denom.ln()).ceil()
}

/// `skimage.measure.ransac((src, dst), SimpleAffineTransform, min_samples=3,
/// residual_threshold=25, max_trials=50, random_state=42)[1]`
pub fn ransac_inliers(src: &[[f64; 2]], dst: &[[f64; 2]]) -> Option<Vec<bool>> {
    const MIN_SAMPLES: usize = 3;
    let num_samples = src.len();
    assert!(MIN_SAMPLES < num_samples);
    let residual_threshold = 25.0;
    let mut max_trials = 50.0f64;
    let mut best_inlier_num = 0usize;
    let mut best_sum = f64::INFINITY;
    let mut best_inliers: Vec<bool> = Vec::new();
    let mut rng = Pcg64::default_rng(42);
    let mut spl = rng.choice_no_replace(num_samples, MIN_SAMPLES);
    let mut num_trials = 0.0f64;
    while num_trials < max_trials {
        num_trials += 1.0;
        let s_src: Vec<[f64; 2]> = spl.iter().map(|&k| src[k]).collect();
        let s_dst: Vec<[f64; 2]> = spl.iter().map(|&k| dst[k]).collect();
        spl = rng.choice_no_replace(num_samples, MIN_SAMPLES);
        let model = Affine::estimate(&s_src, &s_dst);
        let residuals: Vec<f64> = model.residuals(src, dst).into_iter().map(f64::abs).collect();
        let inliers: Vec<bool> = residuals.iter().map(|&r| r < residual_threshold).collect();
        let residuals_sum = blas_dot_self(&residuals);
        let count = inliers.iter().filter(|&&b| b).count();
        if count > best_inlier_num || (count == best_inlier_num && residuals_sum < best_sum) {
            best_inlier_num = count;
            best_sum = residuals_sum;
            best_inliers = inliers;
            let dm = dynamic_max_trials(best_inlier_num, num_samples, MIN_SAMPLES as i32, 1.0);
            // Python min(): keeps the first argument unless the second is smaller
            if dm < max_trials {
                max_trials = dm;
            }
            if best_sum <= 0.0 {
                break;
            }
        }
    }
    if best_inliers.iter().any(|&b| b) {
        Some(best_inliers)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_tokens() {
        assert_eq!(norm_same_token("\\leq"), "\\le");
        assert_eq!(norm_same_token("\\bigl("), "(");
        assert_eq!(norm_same_token("\\Bigg\\langle"), "\\langle");
        assert_eq!(norm_same_token("\\left("), "(");
        assert_eq!(norm_same_token("\\Longrightarrow"), "\\rightarrow");
        assert_eq!(norm_same_token("\\varphi"), "phi");
        assert_eq!(norm_same_token("\\widehat"), "\\hat");
    }
}
