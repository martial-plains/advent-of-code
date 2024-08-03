use md5::{Digest, Md5};

pub const TITLE: &str = "One-Time Pad";

pub const INPUT: &str = "ihaygndm";

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> usize {
    let mut finder = HashFinder::new(input.trim(), 1000, 0);
    let mut index = 0usize;
    for _ in 0..64 {
        index = finder.next();
    }
    index
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    let mut finder = HashFinder::new(input.trim(), 1000, 2016);
    let mut index = 0usize;
    for _ in 0..64 {
        index = finder.next();
    }
    index
}

#[derive(Clone)]
struct Hash {
    value: [u8; 16],   // MD5 hash of salt + index
    count: [u8; 16],   // Max consecutive 0-F in hash
    tripl: Option<u8>, // First triplet, if one exists
}

struct HashFinder {
    key: String,    // Key/salt for creating each Hash
    stt: usize,     // Hash stretching parameter
    idx: usize,     // Next index to be searched
    buf: Vec<Hash>, // Circular buffer of Hash objects
}

fn to_digits(val: &[u8; 16]) -> [u8; 32] {
    let mut digits = [0u8; 32];
    for n in 0..16 {
        digits[2 * n] = (val[n] >> 4) & 0xF;
        digits[2 * n + 1] = val[n] & 0xF;
    }
    digits
}

fn to_hexstr(val: &[u8; 16]) -> [u8; 32] {
    let mut hexstr = to_digits(val);
    for x in &mut hexstr {
        *x = if *x < 10 { *x + 0x30 } else { *x + 0x57 };
    }
    hexstr
}

impl Hash {
    fn new(key: &str, index: usize, stretch: usize) -> Self {
        // Calculate the initial MD5 hash.
        let salt = format!("{key}{index}");
        let mut hash = Self {
            value: compute_md5(salt.as_bytes()).try_into().unwrap(),
            count: [0; 16],
            tripl: None,
        };
        // Hash stretching, if applicable...
        for _ in 0..stretch {
            hash.value = compute_md5(&to_hexstr(&hash.value)).try_into().unwrap();
        }
        // Count consecutive hexadecimal digits.
        let digits = to_digits(&hash.value);
        let mut prev = 0u8;
        let mut count = 0u8;
        for d in digits {
            let n = d as usize;
            count = if d == prev { count + 1 } else { 1 };
            if count >= 3 && hash.tripl.is_none() {
                hash.tripl = Some(d);
            }
            if count >= hash.count[n] {
                hash.count[n] = count;
            }
            prev = d;
        }
        hash
    }

    const fn has3(&self) -> Option<u8> {
        self.tripl
    }

    const fn has5(&self, digit: u8) -> bool {
        self.count[digit as usize] >= 5
    }
}

impl HashFinder {
    fn new(key: &str, search: usize, stretch: usize) -> Self {
        Self {
            key: key.to_string(),
            idx: 0usize,
            stt: stretch,
            buf: (0..=search).map(|n| Hash::new(key, n, stretch)).collect(),
        }
    }

    fn test(&self) -> bool {
        let m = self.idx % self.buf.len();
        if let Some(d) = self.buf[m].has3() {
            for offset in 1..self.buf.len() {
                let n = (self.idx + offset) % self.buf.len();
                if self.buf[n].has5(d) {
                    return true;
                }
            }
        }
        false
    }

    fn incr(&mut self) {
        let n = self.idx % self.buf.len();
        self.buf[n] = Hash::new(&self.key, self.idx + self.buf.len(), self.stt);
        self.idx += 1;
    }

    fn next(&mut self) -> usize {
        while !self.test() {
            self.incr();
        }
        self.incr(); // Get ready for next search...
        self.idx - 1
    }
}

fn compute_md5(to_hash: &[u8]) -> Vec<u8> {
    let mut hash = Md5::new();
    hash.update(to_hash);

    hash.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 15035);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 19968);
    }
}
