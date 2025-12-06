use std::{fs::read_to_string, str::FromStr};

fn sum_of_invalid_ids_in_range(start: u64, end: u64) -> u64 {
    let mut ret = 0u64;

    let start_pow = start.ilog(10) + 1;
    let end_pow = end.ilog(10) + 1;

    for pow in ((start_pow + start_pow % 2)..=(end_pow - end_pow % 2)).step_by(2) {
        let clamped_start = 10_u64.pow(pow - 1).max(start);
        let clamped_end = 10_u64.pow(pow).min(end + 1); // inclusive end

        let mut half_num = clamped_start / 10_u64.pow(pow / 2);
        loop {
            let full_num = half_num * 10_u64.pow(pow / 2) + half_num;
            if full_num >= clamped_end {
                break;
            }
            if full_num < clamped_start {
                half_num += 1;
                continue;
            }
            ret += full_num as u64;
            half_num += 1;
        }
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
        // assert_eq!(sum_of_invalid_ids_in_range(11, 22), 33);
        // assert_eq!(sum_of_invalid_ids_in_range(95, 115), 99);
        // assert_eq!(sum_of_invalid_ids_in_range(95, 1012), 1109);
        // assert_eq!(sum_of_invalid_ids_in_range(1188511880, 1188511890), 1188511885);
        // assert_eq!(sum_of_invalid_ids_in_range(222220, 222224), 222222);
        // assert_eq!(sum_of_invalid_ids_in_range(1698522, 1698528), 0);
        // assert_eq!(sum_of_invalid_ids_in_range(446443, 446449), 446446);
        // assert_eq!(sum_of_invalid_ids_in_range(38593856, 38593862), 38593859);
        assert_eq!(sum_of_invalid_ids_in_range(565653, 565659), 0);
    }
}
