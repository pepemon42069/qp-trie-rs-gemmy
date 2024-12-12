use core::borrow::Borrow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TrieKey {
    value: [u8; 8],
}

impl Borrow<[u8]> for TrieKey {
    fn borrow(&self) -> &[u8] {
        &self.value
    }
}

impl TrieKey {
    pub fn encode(key: u64) -> Self {
        TrieKey {
            value: key.to_be_bytes(),
        }
    }
    pub fn decode(&self) -> u64 {
        u64::from_be_bytes(self.value)
    }
}

#[cfg(test)]
mod tests {
    use Trie;
    use crate::triekey::TrieKey;

    #[test]
    fn test_encode_decode() {
        let value = TrieKey::encode(1000);
        assert_eq!(1000, value.decode())
    }

    #[test]
    fn test_compare_copy() {
        let value = TrieKey::encode(1000);
        let value2 = value;
        let value3 = TrieKey::encode(1001);
        let value4 = TrieKey::encode(999);
        assert!(value == value2 && value != value3 && value2 < value3 && value3 > value4)
    }
}