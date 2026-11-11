//! Exact Levenshtein (unit-cost insert/delete/substitute) distance, as
//! `Levenshtein.distance` (python-Levenshtein / rapidfuzz) computes it, using the
//! bit-parallel algorithm of Myers/Hyyrö. Sequences are `u32` symbols (code points).

use std::collections::HashMap;

/// Precomputed match masks of a pattern for repeated comparisons against many texts.
pub struct Pattern {
    len: usize,
    words: usize,
    ascii: Vec<u64>, // 128 * words
    other: HashMap<u32, Vec<u64>>,
}

impl Pattern {
    pub fn new(p: &[u32]) -> Pattern {
        let len = p.len();
        let words = (len + 63) / 64;
        let mut ascii = vec![0u64; 128 * words.max(1)];
        let mut other: HashMap<u32, Vec<u64>> = HashMap::new();
        for (i, &c) in p.iter().enumerate() {
            let (w, b) = (i / 64, i % 64);
            if c < 128 {
                ascii[c as usize * words + w] |= 1u64 << b;
            } else {
                other.entry(c).or_insert_with(|| vec![0u64; words])[w] |= 1u64 << b;
            }
        }
        Pattern { len, words, ascii, other }
    }

    #[inline]
    fn mask(&self, c: u32, w: usize) -> u64 {
        if c < 128 {
            self.ascii[c as usize * self.words + w]
        } else {
            match self.other.get(&c) {
                Some(v) => v[w],
                None => 0,
            }
        }
    }

    /// Distance between the pattern and `t`.
    pub fn distance(&self, t: &[u32]) -> usize {
        let m = self.len;
        if m == 0 {
            return t.len();
        }
        if t.is_empty() {
            return m;
        }
        if self.words == 1 {
            let mut vp: u64 = !0;
            let mut vn: u64 = 0;
            let last = 1u64 << (m - 1);
            let mut score = m;
            for &c in t {
                let x = self.mask(c, 0) | vn;
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
            return score;
        }
        let words = self.words;
        let mut vp = vec![!0u64; words];
        let mut vn = vec![0u64; words];
        let last = 1u64 << ((m - 1) % 64);
        let mut score = m;
        for &c in t {
            let mut hp_carry: u64 = 1;
            let mut hn_carry: u64 = 0;
            let mut add_carry: u64 = 0;
            for w in 0..words {
                let pm = self.mask(c, w);
                let vpw = vp[w];
                let vnw = vn[w];
                let x = pm | vnw;
                // ((x & vp) + vp + carry) with carry out
                let a = x & vpw;
                let (s1, c1) = a.overflowing_add(vpw);
                let (s2, c2) = s1.overflowing_add(add_carry);
                add_carry = (c1 || c2) as u64;
                let d0 = (s2 ^ vpw) | x;
                let hp = vnw | !(d0 | vpw);
                let hn = vpw & d0;
                if w == words - 1 {
                    if hp & last != 0 {
                        score += 1;
                    } else if hn & last != 0 {
                        score -= 1;
                    }
                }
                let hp_s = (hp << 1) | hp_carry;
                hp_carry = hp >> 63;
                let hn_s = (hn << 1) | hn_carry;
                hn_carry = hn >> 63;
                vp[w] = hn_s | !(d0 | hp_s);
                vn[w] = hp_s & d0;
            }
        }
        score
    }
}

/// Levenshtein distance between two symbol sequences.
pub fn distance(a: &[u32], b: &[u32]) -> usize {
    // common prefix / suffix do not change the distance
    let mut s = 0;
    while s < a.len() && s < b.len() && a[s] == b[s] {
        s += 1;
    }
    let (a, b) = (&a[s..], &b[s..]);
    let mut e = 0;
    while e < a.len() && e < b.len() && a[a.len() - 1 - e] == b[b.len() - 1 - e] {
        e += 1;
    }
    let (a, b) = (&a[..a.len() - e], &b[..b.len() - e]);
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }
    // pattern = shorter sequence
    if a.len() <= b.len() {
        Pattern::new(a).distance(b)
    } else {
        Pattern::new(b).distance(a)
    }
}

pub fn chars(s: &str) -> Vec<u32> {
    s.chars().map(|c| c as u32).collect()
}

/// `Levenshtein.distance(a, b)` on str.
pub fn str_distance(a: &str, b: &str) -> usize {
    distance(&chars(a), &chars(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn naive(a: &[u32], b: &[u32]) -> usize {
        let mut prev: Vec<usize> = (0..=b.len()).collect();
        for i in 1..=a.len() {
            let mut cur = vec![i; b.len() + 1];
            for j in 1..=b.len() {
                cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + (a[i - 1] != b[j - 1]) as usize);
            }
            prev = cur;
        }
        prev[b.len()]
    }
    #[test]
    fn random() {
        let mut seed: u64 = 12345;
        let mut rnd = || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };
        for _ in 0..3000 {
            let la = (rnd() % 200) as usize;
            let lb = (rnd() % 200) as usize;
            let alpha = 1 + rnd() % 5;
            let a: Vec<u32> = (0..la).map(|_| (rnd() % alpha) as u32 + if rnd() % 7 == 0 { 300 } else { 97 }).collect();
            let b: Vec<u32> = (0..lb).map(|_| (rnd() % alpha) as u32 + if rnd() % 7 == 0 { 300 } else { 97 }).collect();
            assert_eq!(distance(&a, &b), naive(&a, &b));
            assert_eq!(Pattern::new(&a).distance(&b), naive(&a, &b));
        }
    }
}
