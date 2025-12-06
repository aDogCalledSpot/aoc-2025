use std::fs::read_to_string;

fn max_joltage_for_line(line: &str) -> u32 {
    let mut positions = vec![None; 10];

    for (i, b) in line.bytes().enumerate() {
        let curr_positions = &mut positions[(b - b'0') as usize];
        *curr_positions = match curr_positions {
            None => Some((i, i)),
            Some((old_first, _old_last)) => Some((*old_first, i)),
        };
    }

    for (largest, first_pos) in positions
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(|(first, _)| (i, first)))
        .rev()
    {
        let Some(second_largest) = positions
            .iter()
            .rposition(|x| x.is_some_and(|(_, last_pos)| last_pos > first_pos))
        else {
            continue;
        };
        return (largest * 10 + second_largest) as u32;
    }
    unreachable!()
}

fn main() {
    let ret: u32 = read_to_string("inputs/day3.txt")
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
        assert_eq!(max_joltage_for_line("811111111111119"), 89);
        assert_eq!(max_joltage_for_line("811111111111118"), 88);
    }
}
