pub mod bitset {

pub struct BitSet {
    data: Vec<u64>
}

struct Indices {
    bucket: usize,
    value: usize
}

impl BitSet {
    /// Constructs a new BitSet with the given size.
    pub fn new(size: usize) -> BitSet {
        BitSet{ data: vec![0; size] }
    }

    fn get_indices(index: usize) -> Indices {
        Indices { bucket: index / 64, value: index % 64 }
    }

    fn get_bucket_mut(&mut self, index: usize) -> &mut u64 {
        &mut self.data[index]
    }

    fn get_bucket(&self, index: usize) -> u64 {
        self.data[index]
    }

    fn get_bit(value: u64, index: usize) -> bool {
        ((value >> index) & 1) == 1
    }

    fn set_bit(value: &mut u64, index: usize) {
        *value |= 1 << index
    }

    fn unset_bit(value: &mut u64, index: usize) {
        *value |= !(1 << index)
    }

    /// Sets the bit at the given index.
    pub fn set(&mut self, index: usize) {
        let indices = BitSet::get_indices(index);
        BitSet::set_bit(self.get_bucket_mut(indices.bucket), indices.value);
    }

    /// Unsets the bit at the given index.
    pub fn unset(&mut self, index: usize) {
        let indices = BitSet::get_indices(index);
        BitSet::unset_bit(self.get_bucket_mut(indices.bucket), indices.value);
    }

    /// Returns the value of the bit at the given index.
    pub fn get(&self, index: usize) -> bool {
        let indices = BitSet::get_indices(index);
        BitSet::get_bit(self.get_bucket(indices.bucket), indices.value)
    }

    /// Clears all bits in the BitSet.
    pub fn clear(&mut self) {
        self.data.fill(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setting() {
        let mut bit_set = BitSet::new(8);
        assert!(!bit_set.get(4));
        bit_set.set(1);
        bit_set.set(4);
        assert!(bit_set.get(1));
        assert!(bit_set.get(4));
        assert!(!bit_set.get(5));
        bit_set.unset(4);
        assert!(bit_set.get(4));
    }

    #[test]
    fn assumptions() {
        assert!(0 / 64 == 0);
        assert!(1 / 64 == 0);
        assert!(63 / 64 == 0);
        assert!(64 / 64 == 1);
        assert!(65 / 64 == 1);
    }
}

}