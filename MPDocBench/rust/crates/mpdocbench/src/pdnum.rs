//! The floating-point summation orders of the numpy 1.24 / pandas 2.0 reductions the
//! scorer uses, so aggregates match bit for bit.

/// numpy `pairwise_sum` (as used by `np.add.reduce` on a contiguous float64 array,
/// which starts from the identity 0.0).
pub fn np_sum(a: &[f64]) -> f64 {
    0.0 + pairwise(a)
}

fn pairwise(a: &[f64]) -> f64 {
    let n = a.len();
    if n < 8 {
        let mut res = 0.0;
        for &x in a {
            res += x;
        }
        res
    } else if n <= 128 {
        let mut r = [a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]];
        let mut i = 8;
        while i < n - (n % 8) {
            for j in 0..8 {
                r[j] += a[i + j];
            }
            i += 8;
        }
        let mut res = ((r[0] + r[1]) + (r[2] + r[3])) + ((r[4] + r[5]) + (r[6] + r[7]));
        while i < n {
            res += a[i];
            i += 1;
        }
        res
    } else {
        let mut n2 = n / 2;
        n2 -= n2 % 8;
        pairwise(&a[..n2]) + pairwise(&a[n2..])
    }
}

/// pandas `Series.sum()` (skipna): NaNs replaced by 0, then numpy's sum.
pub fn pd_sum(a: &[f64]) -> f64 {
    if a.iter().any(|x| x.is_nan()) {
        let v: Vec<f64> = a.iter().map(|&x| if x.is_nan() { 0.0 } else { x }).collect();
        np_sum(&v)
    } else {
        np_sum(a)
    }
}

/// pandas `Series.mean()` (nanops.nanmean, no bottleneck).
pub fn pd_mean(a: &[f64]) -> f64 {
    let count = a.iter().filter(|x| !x.is_nan()).count();
    if count == 0 {
        return f64::NAN;
    }
    pd_sum(a) / count as f64
}

/// pandas cython `group_mean` (Kahan-compensated) over values in row order.
pub fn pd_group_mean(a: &[f64]) -> f64 {
    let mut sumx = 0.0f64;
    let mut comp = 0.0f64;
    let mut nobs = 0usize;
    for &val in a {
        if !val.is_nan() {
            nobs += 1;
            let y = val - comp;
            let t = sumx + y;
            comp = t - sumx - y;
            sumx = t;
        }
    }
    if nobs == 0 {
        f64::NAN
    } else {
        sumx / nobs as f64
    }
}

/// Python `sum(xs) / len(xs)` (sequential, starting from int 0).
pub fn py_mean(a: &[f64]) -> f64 {
    let mut s = 0.0;
    for &x in a {
        s += x;
    }
    s / a.len() as f64
}
