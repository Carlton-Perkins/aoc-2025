use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day2-part1.txt")
        .trim()
        .to_string();
    let mut score = 0;
    let ranges = input.split(|x| x == ',');
    for range in ranges.clone() {
        score += count_invalid(range.to_string()).into_iter().sum::<i64>();
    }
    println!("Part1 Score: {}", score);

    let mut complex_score = 0;
    let ranges = input.split(|x| x == ',');
    for range in ranges.clone() {
        complex_score += count_complex_invalid(range.to_string())
            .into_iter()
            .sum::<i64>();
    }
    println!("Part2 Score: {}", complex_score);

    Ok(())
}

fn count_invalid(input: String) -> Vec<i64> {
    let parts: Vec<&str> = input.split('-').collect();
    let start = parts[0].parse::<i64>().unwrap();
    let end = parts[1].parse::<i64>().unwrap();

    let mut invalid = Vec::new();
    for number in start..=end {
        let mut num_str = number.to_string();
        let rest = num_str.split_off(num_str.len() / 2);
        if num_str == rest {
            invalid.push(number);
        }
    }
    return invalid;
}

fn count_complex_invalid(input: String) -> Vec<i64> {
    let parts: Vec<&str> = input.split('-').collect();
    let start = parts[0].parse::<i64>().unwrap();
    let end = parts[1].parse::<i64>().unwrap();

    let mut invalid = Vec::new();
    for number in start..=end {
        let num_str = number.to_string();
        let mut window = num_str.len() / 2;

        while window > 0 {
            if num_str.len() % window != 0 {
                window -= 1;
                continue;
            }
            let chunks = num_str.chars().chunks(window);
            if chunks
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .all_equal()
            {
                invalid.push(number);
                break;
            }
            window -= 1;
        }
    }
    return invalid;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case() {
        let tests = vec![
            ("11-22", 2),
            ("95-115", 1),
            ("998-1012", 1),
            ("1188511880-1188511890", 1),
            ("222220-222224", 1),
            ("1698522-1698528", 0),
            ("446443-446449", 1),
            ("38593856-38593862", 1),
        ];

        for (input, expected) in tests {
            let invalid = count_invalid(input.into());
            assert_eq!(invalid.len(), expected);
        }
    }
    #[test]
    fn test_complex_example_case() {
        let tests = vec![
            ("11-22", 2),
            ("95-115", 2),
            ("998-1012", 2),
            ("1188511880-1188511890", 1),
            ("222220-222224", 1),
            ("1698522-1698528", 0),
            ("446443-446449", 1),
            ("38593856-38593862", 1),
            ("565653-565659", 1),
            ("824824821-824824827", 1),
            ("2121212118-2121212124", 1),
        ];

        for (input, expected) in tests {
            let invalid = count_complex_invalid(input.into());
            assert_eq!(
                invalid.len(),
                expected,
                "Failed on input: {}, invalid: {:?}",
                input,
                invalid
            );
        }
    }
}
