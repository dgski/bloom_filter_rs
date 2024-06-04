pub mod bloom_filter {
    use crate::bitset;
    use std::hash::{DefaultHasher, Hash, Hasher};


pub struct BloomFilter {
    buckets: bitset::bitset::BitSet
}

impl BloomFilter {

    fn hash(key: &str) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn get_segment_hash(full_hash: u64, index: u64) -> usize {
        ((full_hash >> (16 * index)) as u16) as usize
    }

    pub fn new() -> BloomFilter {
        BloomFilter {
            buckets: bitset::bitset::BitSet::new(u16::MAX as usize)
        }
    }

    pub fn add(&mut self, key: &str) {
        let full_hash = BloomFilter::hash(key);
        let seg_hash = BloomFilter::get_segment_hash;
        self.buckets.set(seg_hash(full_hash, 0));
        self.buckets.set(seg_hash(full_hash, 1));
        self.buckets.set(seg_hash(full_hash, 2));
        self.buckets.set(seg_hash(full_hash, 3));
    }

    pub fn contains(&self, key: &str) -> bool {
        let full_hash = BloomFilter::hash(key);
        let seg_hash = BloomFilter::get_segment_hash;
        self.buckets.get(seg_hash(full_hash, 0)) &
        self.buckets.get(seg_hash(full_hash, 1)) &
        self.buckets.get(seg_hash(full_hash, 2)) &
        self.buckets.get(seg_hash(full_hash, 3))
    }

    pub fn clear(&mut self) {
        self.buckets.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut filter = BloomFilter::new();
        filter.add("hello");
        assert!(filter.contains("hello"));
        assert!(!filter.contains("world"));
    }
}
}