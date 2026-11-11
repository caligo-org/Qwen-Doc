#![allow(clippy::needless_range_loop)]
//! Port of scipy 1.10 `scipy.optimize.linear_sum_assignment` (rectangular_lsap.cpp,
//! Crouse's shortest augmenting path), including its tie-breaking, so that the
//! returned assignment is the same as scipy's when several optima exist.

/// `cost` is row-major `nr x nc`. Returns (row_ind, col_ind) sorted by row, as scipy.
pub fn linear_sum_assignment(cost: &[f64], nr: usize, nc: usize) -> (Vec<usize>, Vec<usize>) {
    if nr == 0 || nc == 0 {
        return (Vec::new(), Vec::new());
    }
    let transpose = nc < nr;
    let (nr, nc, c): (usize, usize, Vec<f64>) = if transpose {
        let mut t = vec![0.0; nr * nc];
        for i in 0..nr {
            for j in 0..nc {
                t[j * nr + i] = cost[i * nc + j];
            }
        }
        (nc, nr, t)
    } else {
        (nr, nc, cost.to_vec())
    };

    let mut u = vec![0.0f64; nr];
    let mut v = vec![0.0f64; nc];
    let mut spc = vec![0.0f64; nc];
    let mut path = vec![usize::MAX; nc];
    let mut col4row = vec![usize::MAX; nr];
    let mut row4col = vec![usize::MAX; nc];
    let mut sr = vec![false; nr];
    let mut sc = vec![false; nc];
    let mut remaining = vec![0usize; nc];

    for cur_row in 0..nr {
        // augmenting_path
        let mut min_val = 0.0f64;
        let mut num_remaining = nc;
        for it in 0..nc {
            remaining[it] = nc - it - 1;
        }
        sr.iter_mut().for_each(|x| *x = false);
        sc.iter_mut().for_each(|x| *x = false);
        spc.iter_mut().for_each(|x| *x = f64::INFINITY);
        let mut i = cur_row;
        let sink;
        loop {
            let mut index = usize::MAX;
            let mut lowest = f64::INFINITY;
            sr[i] = true;
            for it in 0..num_remaining {
                let j = remaining[it];
                let r = min_val + c[i * nc + j] - u[i] - v[j];
                if r < spc[j] {
                    path[j] = i;
                    spc[j] = r;
                }
                if spc[j] < lowest || (spc[j] == lowest && row4col[j] == usize::MAX) {
                    lowest = spc[j];
                    index = it;
                }
            }
            min_val = lowest;
            if min_val == f64::INFINITY {
                panic!("cost matrix is infeasible");
            }
            let j = remaining[index];
            sc[j] = true;
            num_remaining -= 1;
            remaining[index] = remaining[num_remaining];
            if row4col[j] == usize::MAX {
                sink = j;
                break;
            } else {
                i = row4col[j];
            }
        }

        u[cur_row] += min_val;
        for i in 0..nr {
            if sr[i] && i != cur_row {
                u[i] += min_val - spc[col4row[i]];
            }
        }
        for j in 0..nc {
            if sc[j] {
                v[j] -= min_val - spc[j];
            }
        }
        let mut j = sink;
        loop {
            let i = path[j];
            row4col[j] = i;
            std::mem::swap(&mut col4row[i], &mut j);
            if i == cur_row {
                break;
            }
        }
    }

    if transpose {
        let mut idx: Vec<usize> = (0..nr).collect();
        idx.sort_by_key(|&k| col4row[k]);
        let a = idx.iter().map(|&k| col4row[k]).collect();
        (a, idx)
    } else {
        ((0..nr).collect(), col4row)
    }
}
