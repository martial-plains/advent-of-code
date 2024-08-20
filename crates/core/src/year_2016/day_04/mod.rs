pub const TITLE: &str = "Security Through Obscurity";

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
pub struct Room<'a> {
    name: &'a str,
    sector_id: u32,
}

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part1(input: &str) -> u32 {
    let rooms = parse(input).unwrap();
    rooms.iter().map(|room| room.sector_id).sum()
}

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part2(input: &str) -> u32 {
    let rooms = parse(input).unwrap();
    for Room { name, sector_id } in rooms {
        // Check if the name has the correct format
        if name.len() == 24 && name.as_bytes()[9] == b'-' && name.as_bytes()[16] == b'-' {
            let decrypted_name = decrypt_name(name, sector_id);
            if decrypted_name == "northpole object storage" {
                return sector_id;
            }
        }
    }
    unreachable!()
}

fn parse(input: &str) -> anyhow::Result<Vec<Room<'_>>> {
    let mut valid_rooms = Vec::new();
    let to_index = |b: u8| (b - b'a') as usize;

    for line in input.lines() {
        let (name, sector_id, checksum) = split_line(line)?;

        let (freq, fof, highest_freq) = calculate_frequencies(name, to_index);

        if freq[to_index(checksum[0])] != highest_freq {
            continue;
        }

        if !is_valid_checksum(checksum, &freq, &fof, to_index) {
            continue;
        }

        valid_rooms.push(Room { name, sector_id });
    }

    Ok(valid_rooms)
}

fn split_line(line: &str) -> anyhow::Result<(&str, u32, &[u8])> {
    let size = line.len();
    let name = &line[..size - 11];
    let sector_id = line[size - 10..size - 7].parse::<u32>()?;
    let checksum = line[size - 6..size - 1].as_bytes();
    Ok((name, sector_id, checksum))
}

fn calculate_frequencies(
    name: &str,
    to_index: impl Fn(u8) -> usize,
) -> ([usize; 26], [isize; 64], usize) {
    let mut freq = [0; 26];
    let mut fof = [0; 64];
    let mut highest = 0;

    for b in name.bytes() {
        if b != b'-' {
            let index = to_index(b);
            let current = freq[index];
            let next = freq[index] + 1;

            freq[index] = next;
            fof[current] -= 1;
            fof[next] += 1;

            highest = highest.max(next);
        }
    }

    (freq, fof, highest)
}

fn is_valid_checksum(
    checksum: &[u8],
    freq: &[usize; 26],
    fof: &[isize; 64],
    to_index: impl Fn(u8) -> usize,
) -> bool {
    for w in checksum.windows(2) {
        let end = freq[to_index(w[0])];
        let start = freq[to_index(w[1])];

        if start > end || (start == end && w[1] <= w[0]) {
            return false;
        }

        if (start + 1..end).any(|i| fof[i] != 0) {
            return false;
        }
    }

    true
}

/// Decrypts the room name using the sector id
fn decrypt_name(name: &str, sector_id: u32) -> String {
    let mut buffer = String::with_capacity(24);
    for b in name.bytes() {
        buffer.push(decrypt_char(b, sector_id));
    }
    buffer
}

/// Decrypts a single character using the sector id
const fn decrypt_char(b: u8, sector_id: u32) -> char {
    if b == b'-' {
        ' ' // Replace '-' with space
    } else {
        let rotate = (sector_id % 26) as u8;
        let decrypted = (b - b'a' + rotate) % 26 + b'a';
        decrypted as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 158_835);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 993);
    }
}
