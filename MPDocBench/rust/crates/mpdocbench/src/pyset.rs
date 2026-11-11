//! CPython 3.11 `set` of small non-negative ints, reproducing the iteration order
//! (hash table slot order) of the operations the matcher uses. The order matters:
//! the matcher builds an assignment cost matrix in set iteration order, and
//! `linear_sum_assignment` breaks ties by row/column order.

const LINEAR_PROBES: usize = 9;
const PERTURB_SHIFT: u32 = 5;

#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Empty,
    Dummy,
    Key(i64),
}

#[derive(Clone)]
pub struct IntSet {
    table: Vec<Slot>,
    mask: usize,
    fill: usize,
    used: usize,
}

fn hash(k: i64) -> usize {
    // hash(int) == int for small non-negative ints
    debug_assert!(k >= 0);
    k as usize
}

impl IntSet {
    pub fn new() -> IntSet {
        IntSet { table: vec![Slot::Empty; 8], mask: 7, fill: 0, used: 0 }
    }

    /// `set(range(n))`
    pub fn from_range(n: usize) -> IntSet {
        let mut s = IntSet::new();
        for i in 0..n {
            s.add(i as i64);
        }
        s
    }

    pub fn from_iter<I: IntoIterator<Item = i64>>(it: I) -> IntSet {
        let mut s = IntSet::new();
        for k in it {
            s.add(k);
        }
        s
    }

    pub fn len(&self) -> usize {
        self.used
    }

    fn insert_clean(table: &mut [Slot], mask: usize, key: i64) {
        let h = hash(key);
        let mut perturb = h;
        let mut i = h & mask;
        loop {
            if table[i] == Slot::Empty {
                table[i] = Slot::Key(key);
                return;
            }
            if i + LINEAR_PROBES <= mask {
                for j in 1..=LINEAR_PROBES {
                    if table[i + j] == Slot::Empty {
                        table[i + j] = Slot::Key(key);
                        return;
                    }
                }
            }
            perturb >>= PERTURB_SHIFT;
            i = (i.wrapping_mul(5).wrapping_add(1).wrapping_add(perturb)) & mask;
        }
    }

    fn resize(&mut self, minused: usize) {
        let mut newsize = 8usize;
        while newsize <= minused {
            newsize <<= 1;
        }
        let old = std::mem::replace(&mut self.table, vec![Slot::Empty; newsize]);
        self.mask = newsize - 1;
        self.fill = self.used;
        for s in old {
            if let Slot::Key(k) = s {
                Self::insert_clean(&mut self.table, self.mask, k);
            }
        }
    }

    pub fn contains(&self, key: i64) -> bool {
        let h = hash(key);
        let mut perturb = h;
        let mut i = h & self.mask;
        loop {
            let probes = if i + LINEAR_PROBES <= self.mask { LINEAR_PROBES } else { 0 };
            for j in 0..=probes {
                match self.table[i + j] {
                    Slot::Empty => return false,
                    Slot::Key(k) if k == key => return true,
                    _ => {}
                }
            }
            perturb >>= PERTURB_SHIFT;
            i = (i.wrapping_mul(5).wrapping_add(1).wrapping_add(perturb)) & self.mask;
        }
    }

    pub fn add(&mut self, key: i64) {
        let h = hash(key);
        let mut perturb = h;
        let mask = self.mask;
        let mut i = h & mask;
        loop {
            let probes = if i + LINEAR_PROBES <= mask { LINEAR_PROBES } else { 0 };
            for j in 0..=probes {
                match self.table[i + j] {
                    Slot::Empty => {
                        self.table[i + j] = Slot::Key(key);
                        self.fill += 1;
                        self.used += 1;
                        if self.fill * 5 < mask * 3 {
                            return;
                        }
                        let n = if self.used > 50000 { self.used * 2 } else { self.used * 4 };
                        self.resize(n);
                        return;
                    }
                    Slot::Key(k) if k == key => return,
                    _ => {}
                }
            }
            perturb >>= PERTURB_SHIFT;
            i = (i.wrapping_mul(5).wrapping_add(1).wrapping_add(perturb)) & mask;
        }
    }

    fn discard(&mut self, key: i64) {
        let h = hash(key);
        let mut perturb = h;
        let mut i = h & self.mask;
        loop {
            let probes = if i + LINEAR_PROBES <= self.mask { LINEAR_PROBES } else { 0 };
            for j in 0..=probes {
                match self.table[i + j] {
                    Slot::Empty => return,
                    Slot::Key(k) if k == key => {
                        self.table[i + j] = Slot::Dummy;
                        self.used -= 1;
                        return;
                    }
                    _ => {}
                }
            }
            perturb >>= PERTURB_SHIFT;
            i = (i.wrapping_mul(5).wrapping_add(1).wrapping_add(perturb)) & self.mask;
        }
    }

    /// Iteration in slot order.
    pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
        self.table.iter().filter_map(|s| if let Slot::Key(k) = s { Some(*k) } else { None })
    }

    pub fn to_vec(&self) -> Vec<i64> {
        self.iter().collect()
    }

    /// `set(self)` — set_copy via set_merge.
    fn copy(&self) -> IntSet {
        let mut s = IntSet::new();
        if (s.fill + self.used) * 5 >= s.mask * 3 {
            s.resize((s.used + self.used) * 2);
        }
        if s.mask == self.mask && self.fill == self.used {
            s.table = self.table.clone();
            s.fill = self.fill;
            s.used = self.used;
            return s;
        }
        s.fill = self.used;
        s.used = self.used;
        let mask = s.mask;
        for slot in &self.table {
            if let Slot::Key(k) = slot {
                Self::insert_clean(&mut s.table, mask, *k);
            }
        }
        s
    }

    /// `self - other` (CPython set_difference with a set argument).
    pub fn difference(&self, other: &IntSet) -> IntSet {
        if self.used == 0 {
            return self.copy();
        }
        if (self.used >> 2) > other.used {
            // set_copy_and_difference
            let mut r = self.copy();
            // (other is small here, so CPython's "other 8x larger" intersection shortcut never applies)
            for k in other.iter() {
                r.discard(k);
            }
            if r.fill - r.used > r.mask / 4 {
                let n = if r.used > 50000 { r.used * 2 } else { r.used * 4 };
                r.resize(n);
            }
            return r;
        }
        let mut r = IntSet::new();
        for k in self.iter() {
            if !other.contains(k) {
                r.add(k);
            }
        }
        r
    }
}
