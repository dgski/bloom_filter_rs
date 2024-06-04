pub mod bloom_filter {
    use crate::bitset;
    use std::hash::{DefaultHasher, Hash, Hasher};


struct BloomFilter {
    buckets: bitset::bitset::BitSet
}

impl BloomFilter {

    fn hash(key: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn get_segment_hash(full_hash: u64, index: u64) -> u16 {
        (full_hash >> (16 * index)) as u16
    }

    fn new() -> BloomFilter {
        BloomFilter {
            buckets: bitset::bitset::BitSet::new(u16::MAX as usize)
        }
    }

    fn add(&mut self, key: &str) {
        let full_hash = BloomFilter::hash(key);
        self.buckets.set(BloomFilter::get_segment_hash(full_hash, 0) as usize);
        self.buckets.set(BloomFilter::get_segment_hash(full_hash, 1) as usize);
        self.buckets.set(BloomFilter::get_segment_hash(full_hash, 2) as usize);
        self.buckets.set(BloomFilter::get_segment_hash(full_hash, 3) as usize);
    }

    fn contains(&self, key: &str) -> bool {
        let full_hash = BloomFilter::hash(key);
        self.buckets.get(BloomFilter::get_segment_hash(full_hash, 0) as usize) &
        self.buckets.get(BloomFilter::get_segment_hash(full_hash, 1) as usize) &
        self.buckets.get(BloomFilter::get_segment_hash(full_hash, 2) as usize) &
        self.buckets.get(BloomFilter::get_segment_hash(full_hash, 3) as usize)
    }

    fn clear(&mut self) {
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