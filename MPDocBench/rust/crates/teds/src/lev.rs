//! Exact Levenshtein distance (unit insert/delete/substitute) on sequences of `u32` symbols,
//! with a bit-parallel algorithm (Myers 1999 / Hyyrö 2003, block-based for long patterns).

/// A pattern preprocessed for repeated distance computations against many texts.
pub struct Pattern {
    len: usize,
    blocks: usize,
    /// Distinct symbols, sorted, each with `blocks` match-mask words in `masks`.
    symbols: Vec<u32>,
    masks: Vec<u64>,
}

impl Pattern {
    pub fn new(seq: &[u32]) -> Pattern {
        let len = seq.len();
        let blocks = len.div_ceil(64).max(1);
        let mut symbols: Vec<u32> = seq.to_vec();
        symbols.sort_unstable();
        symbols.dedup();
        let mut masks = vec![0u64; symbols.len() * blocks];
        for (pos, s) in seq.iter().enumerate() {
            let k = symbols.binary_search(s).unwrap();
            masks[k * blocks + pos / 64] |= 1u64 << (pos % 64);
        }
        Pattern { len, blocks, symbols, masks }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn lookup(&self, s: u32) -> Option<usize> {
        if self.symbols.len() <= 16 {
            self.symbols.iter().position(|&x| x == s)
        } else {
            self.symbols.binary_search(&s).ok()
        }
    }

    /// Levenshtein distance between the pattern and `text`.
    pub fn distance(&self, text: &[u32]) -> usize {
        let m = self.len;
        if m == 0 {
            return text.len();
        }
        if text.is_empty() {
            return m;
        }
        if self.blocks == 1 {
            return self.distance_single(text);
        }
        self.distance_blocks(text)
    }

    fn distance_single(&self, text: &[u32]) -> usize {
        let m = self.len;
        let last = 1u64 << (m - 1);
        let mut vp: u64 = !0;
        let mut vn: u64 = 0;
        let mut score = m;
        for &c in text {
            let pm = match self.lookup(c) {
                Some(k) => self.masks[k],
                None => 0,
            };
            let x = pm | vn;
            let d0 = ((x & vp).wrapping_add(vp) ^ vp) | x;
            let mut hp = vn | !(d0 | vp);
            let mut hn = vp & d0;
            if hp & last != 0 {
                score += 1;
            } else if hn & last != 0 {
                score -= 1;
            }
            hp = (hp << 1) | 1;
            hn <<= 1;
            vp = hn | !(d0 | hp);
            vn = hp & d0;
        }
        score
    }

    fn distance_blocks(&self, text: &[u32]) -> usize {
        let m = self.len;
        let nb = self.blocks;
        let last_bit = (m - 1) % 64;
        let mut pv = vec![!0u64; nb];
        let mut mv = vec![0u64; nb];
        let mut score = m as i64;
        for &c in text {
            let k = self.lookup(c);
            let mut hin: i32 = 1;
            for b in 0..nb {
                let mut eq = match k {
                    Some(k) => self.masks[k * nb + b],
                    None => 0,
                };
                let p = pv[b];
                let mm = mv[b];
                let xv = eq | mm;
                if hin < 0 {
                    eq |= 1;
                }
                let xh = ((eq & p).wrapping_add(p) ^ p) | eq;
                let mut ph = mm | !(xh | p);
                let mut mh = p & xh;
                let hb = if b + 1 == nb { last_bit } else { 63 };
                let hout = ((ph >> hb) & 1) as i32 - ((mh >> hb) & 1) as i32;
                ph <<= 1;
                mh <<= 1;
                if hin < 0 {
                    mh |= 1;
                } else if hin > 0 {
                    ph |= 1;
                }
                pv[b] = mh | !(xv | ph);
                mv[b] = ph & xv;
                hin = hout;
            }
            score += hin as i64;
        }
        score as usize
    }
}

/// Levenshtein distance between two symbol sequences.
pub fn levenshtein(a: &[u32], b: &[u32]) -> usize {
    // Pattern = the shorter sequence keeps the bit vectors small.
    let (p, t) = if a.len() <= b.len() { (a, b) } else { (b, a) };
    Pattern::new(p).distance(t)
}

/// Reference O(nm) dynamic programme, used by tests.
pub fn levenshtein_dp(a: &[u32], b: &[u32]) -> usize {
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0; b.len() + 1];
    for (i, &x) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, &y) in b.iter().enumerate() {
            let sub = prev[j] + usize::from(x != y);
            cur[j + 1] = sub.min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_dp() {
        let mut state: u64 = 0x1234_5678_9abc_def0;
        let mut rnd = move |n: u64| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state % n
        };
        for _ in 0..3000 {
            let la = rnd(300) as usize;
            let lb = rnd(300) as usize;
            let alpha = 1 + rnd(40);
            let a: Vec<u32> = (0..la).map(|_| rnd(alpha) as u32).collect();
            let mut b: Vec<u32> = (0..lb).map(|_| rnd(alpha) as u32).collect();
            if rnd(3) == 0 {
                b = a.clone();
                for _ in 0..rnd(10) {
                    if !b.is_empty() {
                        let k = rnd(b.len() as u64) as usize;
                        b[k] = rnd(alpha) as u32;
                    }
                }
            }
            assert_eq!(levenshtein(&a, &b), levenshtein_dp(&a, &b), "{a:?} {b:?}");
            assert_eq!(Pattern::new(&a).distance(&b), levenshtein_dp(&a, &b));
        }
    }
}
