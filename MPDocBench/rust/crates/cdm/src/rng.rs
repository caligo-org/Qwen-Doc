//! numpy 1.24 `np.random.default_rng(seed)`: SeedSequence -> PCG64 (XSL-RR 128/64),
//! and the exact draws of `Generator.choice(n, k, replace=False)` (Floyd's algorithm
//! with a tail shuffle, Lemire bounded integers on the buffered 32-bit stream).

const INIT_A: u32 = 0x43b0_d7e5;
const MULT_A: u32 = 0x931e_8875;
const INIT_B: u32 = 0x8b51_f9dd;
const MULT_B: u32 = 0x58f3_8ded;
const MIX_MULT_L: u32 = 0xca01_f9dd;
const MIX_MULT_R: u32 = 0x4973_f715;
const XSHIFT: u32 = 16;
const POOL_SIZE: usize = 4;

fn hashmix(value: u32, hash_const: &mut u32) -> u32 {
    let mut value = value ^ *hash_const;
    *hash_const = hash_const.wrapping_mul(MULT_A);
    value = value.wrapping_mul(*hash_const);
    value ^ (value >> XSHIFT)
}

fn mix(x: u32, y: u32) -> u32 {
    let r = MIX_MULT_L.wrapping_mul(x).wrapping_sub(MIX_MULT_R.wrapping_mul(y));
    r ^ (r >> XSHIFT)
}

/// `SeedSequence(seed).generate_state(n_words, np.uint64)` for a seed < 2**32.
fn seed_sequence_u64(seed: u32, n_words: usize) -> Vec<u64> {
    let entropy = [seed];
    let mut pool = [0u32; POOL_SIZE];
    let mut hash_const = INIT_A;
    for i in 0..POOL_SIZE {
        pool[i] = if i < entropy.len() {
            hashmix(entropy[i], &mut hash_const)
        } else {
            hashmix(0, &mut hash_const)
        };
    }
    for i_src in 0..POOL_SIZE {
        for i_dst in 0..POOL_SIZE {
            if i_src != i_dst {
                let h = hashmix(pool[i_src], &mut hash_const);
                pool[i_dst] = mix(pool[i_dst], h);
            }
        }
    }
    let n32 = n_words * 2;
    let mut hash_const = INIT_B;
    let mut state = vec![0u32; n32];
    for (i_dst, s) in state.iter_mut().enumerate() {
        let mut data_val = pool[i_dst % POOL_SIZE];
        data_val ^= hash_const;
        hash_const = hash_const.wrapping_mul(MULT_B);
        data_val = data_val.wrapping_mul(hash_const);
        data_val ^= data_val >> XSHIFT;
        *s = data_val;
    }
    (0..n_words).map(|k| state[2 * k] as u64 | ((state[2 * k + 1] as u64) << 32)).collect()
}

const PCG_MULT: u128 = (2549297995355413924u128 << 64) + 4865540595714422341u128;

pub struct Pcg64 {
    state: u128,
    inc: u128,
    has_uint32: bool,
    uinteger: u32,
}

impl Pcg64 {
    /// `np.random.default_rng(seed)` bit generator.
    pub fn default_rng(seed: u32) -> Pcg64 {
        let v = seed_sequence_u64(seed, 4);
        let initstate = ((v[0] as u128) << 64) | v[1] as u128;
        let initseq = ((v[2] as u128) << 64) | v[3] as u128;
        let mut r = Pcg64 { state: 0, inc: (initseq << 1) | 1, has_uint32: false, uinteger: 0 };
        r.step();
        r.state = r.state.wrapping_add(initstate);
        r.step();
        r
    }

    fn step(&mut self) {
        self.state = self.state.wrapping_mul(PCG_MULT).wrapping_add(self.inc);
    }

    pub fn next_u64(&mut self) -> u64 {
        self.step();
        let s = self.state;
        let rot = (s >> 122) as u32;
        let v = ((s >> 64) as u64) ^ (s as u64);
        v.rotate_right(rot)
    }

    pub fn next_u32(&mut self) -> u32 {
        if self.has_uint32 {
            self.has_uint32 = false;
            return self.uinteger;
        }
        let next = self.next_u64();
        self.has_uint32 = true;
        self.uinteger = (next >> 32) as u32;
        next as u32
    }

    /// `random_bounded_uint64(state, 0, rng, 0, use_masked=False)`: uniform in [0, rng].
    fn bounded(&mut self, rng: u64) -> u64 {
        if rng == 0 {
            return 0;
        }
        if rng <= 0xFFFF_FFFF {
            if rng == 0xFFFF_FFFF {
                return self.next_u32() as u64;
            }
            let rng32 = rng as u32;
            let rng_excl = rng32 + 1;
            let mut m = (self.next_u32() as u64) * rng_excl as u64;
            let mut leftover = m as u32;
            if leftover < rng_excl {
                let threshold = (u32::MAX - rng32) % rng_excl;
                while leftover < threshold {
                    m = (self.next_u32() as u64) * rng_excl as u64;
                    leftover = m as u32;
                }
            }
            return m >> 32;
        }
        // 64-bit Lemire (not reached for the population sizes CDM uses)
        if rng == u64::MAX {
            return self.next_u64();
        }
        let rng_excl = rng + 1;
        let mut m = (self.next_u64() as u128) * rng_excl as u128;
        let mut leftover = m as u64;
        if leftover < rng_excl {
            let threshold = (u64::MAX - rng) % rng_excl;
            while leftover < threshold {
                m = (self.next_u64() as u128) * rng_excl as u128;
                leftover = m as u64;
            }
        }
        (m >> 64) as u64
    }

    /// `Generator.choice(pop_size, size, replace=False)` (shuffle=True, p=None).
    pub fn choice_no_replace(&mut self, pop_size: usize, size: usize) -> Vec<usize> {
        assert!(size <= pop_size);
        let pop = pop_size as u64;
        let sz = size as u64;
        if pop > 10000 && sz > pop / 50 {
            // tail shuffle
            let mut idx: Vec<i64> = (0..pop as i64).collect();
            let first = std::cmp::max(pop - sz, 1);
            self.shuffle_int(&mut idx, pop, first);
            return idx[(pop - sz) as usize..].iter().map(|&x| x as usize).collect();
        }
        let mut idx = vec![0i64; size];
        let set_size = (1.2 * sz as f64) as u64;
        let mask = gen_mask(set_size);
        let set_size = 1 + mask;
        let mut hash_set = vec![u64::MAX; set_size as usize];
        for j in (pop - sz)..pop {
            let val = self.bounded(j);
            let mut loc = val & mask;
            while hash_set[loc as usize] != u64::MAX && hash_set[loc as usize] != val {
                loc = (loc + 1) & mask;
            }
            if hash_set[loc as usize] == u64::MAX {
                hash_set[loc as usize] = val;
                idx[(j + sz - pop) as usize] = val as i64;
            } else {
                let mut loc = j & mask;
                while hash_set[loc as usize] != u64::MAX {
                    loc = (loc + 1) & mask;
                }
                hash_set[loc as usize] = j;
                idx[(j + sz - pop) as usize] = j as i64;
            }
        }
        self.shuffle_int(&mut idx, sz, 1);
        idx.into_iter().map(|x| x as usize).collect()
    }

    fn shuffle_int(&mut self, data: &mut [i64], n: u64, first: u64) {
        let mut i = n;
        while i > first {
            i -= 1;
            let j = self.bounded(i) as usize;
            data.swap(j, i as usize);
        }
    }
}

fn gen_mask(max: u64) -> u64 {
    let mut mask = max;
    mask |= mask >> 1;
    mask |= mask >> 2;
    mask |= mask >> 4;
    mask |= mask >> 8;
    mask |= mask >> 16;
    mask |= mask >> 32;
    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_matches_numpy() {
        let r = Pcg64::default_rng(42);
        assert_eq!(r.state, 274674114334540486603088602300644985544u128);
        assert_eq!(r.inc, 332724090758049132448979897138935081983u128);
    }

    #[test]
    fn choice_matches_numpy() {
        let txt = include_str!("../tests/data/rng_ref.txt");
        for line in txt.lines() {
            let (n, rest) = line.split_once(':').unwrap();
            let n: usize = n.parse().unwrap();
            let mut r = Pcg64::default_rng(42);
            for s in rest.split(';') {
                let want: Vec<usize> = s.split(',').map(|x| x.parse().unwrap()).collect();
                assert_eq!(r.choice_no_replace(n, 3), want, "n={n}");
            }
        }
    }
}
