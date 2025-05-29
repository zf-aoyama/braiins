use bit_vec::BitVec;

#[derive(Clone, Debug, Default)]
pub struct BitSet {
    bits: BitVec,
}

impl BitSet {
    pub fn new() -> Self {
        Self { bits: BitVec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { bits: BitVec::from_elem(capacity, false) }
    }

    fn ensure(&mut self, index: usize) {
        if index >= self.bits.len() {
            self.bits.grow(index + 1 - self.bits.len(), false);
        }
    }

    pub fn insert(&mut self, index: usize) -> bool {
        self.ensure(index);
        let old = self.bits.get(index).unwrap_or(false);
        self.bits.set(index, true);
        !old
    }

    pub fn remove(&mut self, index: usize) -> bool {
        if index < self.bits.len() {
            let old = self.bits.get(index).unwrap_or(false);
            self.bits.set(index, false);
            old
        } else {
            false
        }
    }

    pub fn contains(&self, index: usize) -> bool {
        self.bits.get(index).unwrap_or(false)
    }

    pub fn clear(&mut self) {
        self.bits.clear();
    }

    pub fn is_empty(&self) -> bool {
        !self.bits.any()
    }

    pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        self.bits.iter().enumerate().filter_map(|(i, b)| if b { Some(i) } else { None })
    }

    pub fn union_with(&mut self, other: &Self) {
        if other.bits.len() > self.bits.len() {
            self.bits.grow(other.bits.len() - self.bits.len(), false);
        }
        for i in other.iter() {
            self.bits.set(i, true);
        }
    }
}

impl FromIterator<usize> for BitSet {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let mut set = BitSet::new();
        for i in iter {
            set.insert(i);
        }
        set
    }
}

impl IntoIterator for BitSet {
    type Item = usize;
    type IntoIter = std::vec::IntoIter<usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter().collect::<Vec<_>>().into_iter()
    }
}
