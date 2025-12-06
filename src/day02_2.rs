use std::{collections::HashSet, fs::read_to_string, str::FromStr};

fn full_num_from_segment(segment: u64, pow: u32, segment_length: u32) -> u64 {
    let mut full_num = segment;
    for _ in 1..pow / segment_length {
        full_num *= 10_u64.pow(segment_length);
        full_num += segment;
    }
    full_num
}

fn sum_of_invalid_id_in_range_with_pow(
    start: u64,
    end: u64,
    pow: u32,
    pruned_nums: &mut HashSet<u64>,
) -> u64 {
    let mut ret = 0u64;

    let clamped_start = 10_u64.pow(pow - 1).max(start);
    let clamped_end = 10_u64.pow(pow).min(end);

    for segment_length in 1..=pow / 2 {
        if pow % segment_length != 0 {
            continue;
        }

        let mut segment = clamped_start / 10_u64.pow(pow - segment_length);
        loop {
            let full_num = full_num_from_segment(segment, pow, segment_length);
            if full_num >= clamped_end {
                break;
            }
            if full_num < clamped_start {
                segment += 1;
                continue;
            }
            if !pruned_nums.contains(&full_num) {
                pruned_nums.insert(full_num);
                ret += full_num;
            }
            segment += 1;
        }
    }
    ret
}

fn sum_of_invalid_ids_in_range(start: u64, end: u64) -> u64 {
    let mut ret = 0u64;

    // I prefer doing this to the linear scan through all numbers
    let mut pruned_nums = HashSet::new();

    let start_pow = start.ilog(10) + 1;
    let end_pow = end.ilog(10) + 1;

    for pow in start_pow..=end_pow {
        let clamped_start = 10_u64.pow(pow - 1).max(start);
        let clamped_end = 10_u64.pow(pow).min(end + 1); // inclusive end

        ret +=
            sum_of_invalid_id_in_range_with_pow(clamped_start, clamped_end, pow, &mut pruned_nums);
    }
    ret
}

fn main() {
    let mut sum = 0;
    for ranges in read_to_string("inputs/day2.txt")
        .unwrap()
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
    {
        let nums = &ranges
            .split('-')
            .map(|s| u64::from_str(s).unwrap())
            .collect::<Vec<_>>();
        sum += sum_of_invalid_ids_in_range(nums[0], nums[1]);
    }
    println!("{sum}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stuff() {
        assert_eq!(sum_of_invalid_ids_in_range(11, 22), 33);
        assert_eq!(sum_of_invalid_ids_in_range(95, 115), 210);
        assert_eq!(
            sum_of_invalid_ids_in_range(1188511880, 1188511890),
            1188511885
        );
        assert_eq!(sum_of_invalid_ids_in_range(222220, 222224), 222222);
        assert_eq!(sum_of_invalid_ids_in_range(1698522, 1698528), 0);
        assert_eq!(sum_of_invalid_ids_in_range(446443, 446449), 446446);
        assert_eq!(sum_of_invalid_ids_in_range(38593856, 38593862), 38593859);
        assert_eq!(sum_of_invalid_ids_in_range(565653, 565659), 565656);
        assert_eq!(sum_of_invalid_ids_in_range(824824821, 824824827), 824824824);
        assert_eq!(
            sum_of_invalid_ids_in_range(2121212118, 2121212124),
            2121212121
        );
    }
}
