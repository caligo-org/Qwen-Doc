//! Exact ordered tree edit distance (Zhang & Shasha 1989).
//!
//! Trees are given as child lists indexed by node id (`children[id]` lists the ids of the
//! children of `id`, in order) plus a root id; only nodes reachable from the root take part,
//! and the reachable part must be a tree (each node reached once). Costs are callbacks on
//! node ids. This is the optimal distance; Python `apted` 1.0.3 (which the MPDocBench metrics
//! use, see [`crate::apted()`]) sometimes returns a larger value.

/// Postorder numbering of the nodes reachable from `root`.
struct Indexed {
    /// postorder index -> node id
    ids: Vec<usize>,
    /// postorder index -> postorder index of its leftmost leaf descendant
    lml: Vec<usize>,
    /// postorder indices of the key roots, ascending
    keyroots: Vec<usize>,
}

fn index(children: &[Vec<usize>], root: usize) -> Indexed {
    let mut ids = Vec::new();
    let mut lml = Vec::new();
    // Iterative postorder: (node, next child position, leftmost leaf of the node).
    let mut stack: Vec<(usize, usize, usize)> = vec![(root, 0, usize::MAX)];
    while let Some(top) = stack.last_mut() {
        let (node, next, _) = *top;
        if next < children[node].len() {
            top.1 += 1;
            stack.push((children[node][next], 0, usize::MAX));
        } else {
            let (_, _, mut leftmost) = stack.pop().unwrap();
            let p = ids.len();
            if leftmost == usize::MAX {
                leftmost = p; // a leaf
            }
            ids.push(node);
            lml.push(leftmost);
            if let Some(parent) = stack.last_mut() {
                if parent.1 == 1 {
                    // `node` was the parent's first child.
                    parent.2 = leftmost;
                }
            }
        }
    }
    let n = ids.len();
    let mut seen = vec![false; n];
    let mut keyroots = Vec::new();
    for p in (0..n).rev() {
        if !seen[lml[p]] {
            seen[lml[p]] = true;
            keyroots.push(p);
        }
    }
    keyroots.reverse();
    Indexed { ids, lml, keyroots }
}

/// Tree edit distance with arbitrary non-negative costs.
pub fn tree_edit_distance<D, I, R>(
    children1: &[Vec<usize>],
    root1: usize,
    children2: &[Vec<usize>],
    root2: usize,
    mut delete: D,
    mut insert: I,
    mut rename: R,
) -> f64
where
    D: FnMut(usize) -> f64,
    I: FnMut(usize) -> f64,
    R: FnMut(usize, usize) -> f64,
{
    let t1 = index(children1, root1);
    let t2 = index(children2, root2);
    let n1 = t1.ids.len();
    let n2 = t2.ids.len();
    let del: Vec<f64> = t1.ids.iter().map(|&id| delete(id)).collect();
    let ins: Vec<f64> = t2.ids.iter().map(|&id| insert(id)).collect();
    let lml1 = &t1.lml;
    let lml2 = &t2.lml;

    let mut td = vec![0f64; n1 * n2];
    let mut fd = vec![0f64; (n1 + 1) * (n2 + 1)];

    for &k1 in &t1.keyroots {
        let l1 = lml1[k1];
        let rows = k1 - l1 + 2;
        for &k2 in &t2.keyroots {
            let l2 = lml2[k2];
            let cols = k2 - l2 + 2;
            fd[0] = 0.0;
            for di in 1..rows {
                fd[di * cols] = fd[(di - 1) * cols] + del[l1 + di - 1];
            }
            for dj in 1..cols {
                fd[dj] = fd[dj - 1] + ins[l2 + dj - 1];
            }
            for di in 1..rows {
                let i = l1 + di - 1;
                let li = lml1[i];
                let row = di * cols;
                let prow = row - cols;
                let lrow = (li - l1) * cols;
                let del_i = del[i];
                let td_row = i * n2;
                let id1 = t1.ids[i];
                for dj in 1..cols {
                    let j = l2 + dj - 1;
                    let lj = lml2[j];
                    let a = fd[prow + dj] + del_i;
                    let b = fd[row + dj - 1] + ins[j];
                    let mut v = if a < b { a } else { b };
                    if li == l1 && lj == l2 {
                        let c = fd[prow + dj - 1] + rename(id1, t2.ids[j]);
                        if c < v {
                            v = c;
                        }
                        fd[row + dj] = v;
                        td[td_row + j] = v;
                    } else {
                        let c = fd[lrow + (lj - l2)] + td[td_row + j];
                        if c < v {
                            v = c;
                        }
                        fd[row + dj] = v;
                    }
                }
            }
        }
    }
    td[n1 * n2 - 1]
}

/// Optimal tree edit distance with insert and delete cost 1.
pub fn unit_tree_edit_distance<R>(
    children1: &[Vec<usize>],
    root1: usize,
    children2: &[Vec<usize>],
    root2: usize,
    rename: R,
) -> f64
where
    R: FnMut(usize, usize) -> f64,
{
    tree_edit_distance(children1, root1, children2, root2, |_| 1.0, |_| 1.0, rename)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ted_labels(c1: &[Vec<usize>], l1: &[u8], c2: &[Vec<usize>], l2: &[u8]) -> f64 {
        unit_tree_edit_distance(c1, 0, c2, 0, |a, b| if l1[a] == l2[b] { 0.0 } else { 1.0 })
    }

    #[test]
    fn classic_example() {
        // Zhang & Shasha: f(d(a c(b)) e) vs f(c(d(a b)) e) has distance 2.
        // ids: 0 f, 1 d, 2 a, 3 c, 4 b, 5 e
        let c1 = vec![vec![1, 5], vec![2, 3], vec![], vec![4], vec![], vec![]];
        let l1 = b"fdacbe";
        // ids: 0 f, 1 c, 2 d, 3 a, 4 b, 5 e
        let c2 = vec![vec![1, 5], vec![2], vec![3, 4], vec![], vec![], vec![]];
        let l2 = b"fcdabe";
        assert_eq!(ted_labels(&c1, l1, &c2, l2), 2.0);
        assert_eq!(ted_labels(&c1, l1, &c1, l1), 0.0);
        let single = vec![vec![]];
        assert_eq!(ted_labels(&c1, l1, &single, b"f"), 5.0);
        assert_eq!(ted_labels(&single, b"x", &c2, l2), 6.0);
    }
}
