use std::fs::read_to_string;

fn build_new_attempt_from_dp(line: &str, pos: usize, length: usize, prev: &[u64]) -> u64 {
    let num_val_at_pos = |pos: usize| -> u64 { (line.as_bytes()[pos] - b'0') as u64 };
    let prev = prev[pos + 1];
    num_val_at_pos(pos) * 10_u64.pow(length as u32) + prev
}

fn max_joltage_for_line(line: &str) -> u64 {
    let num_val_at_pos = |pos: usize| -> u64 { (line.as_bytes()[pos] - b'0') as u64 };
    let mut prev = vec![0; line.len()];

    // init phase
    prev[line.len() - 1] = num_val_at_pos(line.len() - 1);
    for i in (0..(line.len() - 1)).rev() {
        prev[i] = prev[i + 1].max(num_val_at_pos(i));
    }

    for substr_len in 1..12 {
        let mut next = vec![0; line.len()];
        let last_pos_for_substr_len = line.len() - substr_len - 1;
        next[last_pos_for_substr_len] =
            build_new_attempt_from_dp(line, last_pos_for_substr_len, substr_len, &prev);
        for pos in (0..last_pos_for_substr_len).rev() {
            let new_attempt = build_new_attempt_from_dp(line, pos, substr_len, &prev);
            next[pos] = new_attempt.max(next[pos + 1])
        }
        prev = next;
    }
    prev[0]
}

fn main() {
    let ret: u64 = read_to_string("inputs/day3.txt")
        .unwrap()
        .lines()
        .map(max_joltage_for_line)
        .sum();
    println!("{ret}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stuff() {
        assert_eq!(max_joltage_for_line("987654321111111"), 987654321111);
        assert_eq!(max_joltage_for_line("811111111111119"), 811111111119);
        assert_eq!(max_joltage_for_line("234234234234278"), 434234234278);
        assert_eq!(max_joltage_for_line("818181911112111"), 888911112111);
    }
}
