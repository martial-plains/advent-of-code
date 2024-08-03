use md5::{Digest, Md5};

pub const TITLE: &str = "How About a Nice Game of Chess?";

pub const INPUT: &str = "wtnhxymk";

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part1(input: &str) -> String {
    let mut count = 0usize;
    let mut index = 0u64;
    let mut password = 0u64;
    while count < 8 {
        let hash = md5_prefix(input, index) / 256;
        if hash / 16 == 0 {
            count += 1;
            password = 16 * password + (hash % 16);
        }
        index += 1;
    }

    format!("{password:08x}")
}

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part2(input: &str) -> String {
    let mut pmask = 0u8;
    let mut index = 0u64;
    let mut password = 0u64;
    while pmask < 255 {
        let hash = md5_prefix(input, index);
        let pfix = hash & 0xFFFF_F000;
        let posn = (hash & 0x0000_0F00) / 256;
        let pval = (hash & 0x0000_00F0) / 16;
        if (pfix == 0) && (posn < 8) {
            let tmask = 1u8 << posn;
            if pmask & tmask == 0 {
                pmask |= tmask;
                password |= pval << (28 - 4 * posn);
            }
        }
        index += 1;
    }

    format!("{password:08x}")
}

// Return the first four bytes of the MD5 hash.
fn md5_prefix(base: &str, idx: u64) -> u64 {
    let salt = format!("{base}{idx}");
    let hash = compute_md5(salt.as_bytes());
    16_777_216 * u64::from(hash[0])
        + 65536 * u64::from(hash[1])
        + 256 * u64::from(hash[2])
        + u64::from(hash[3])
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
        assert_eq!(part1(INPUT), "2414bc77");
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), "437e60fc");
    }
}
