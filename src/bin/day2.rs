use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day2-part1.txt")
        .trim()
        .to_string();
    let mut score = 0;
    let ranges = input.split(|x| x == ',');
    for range in ranges {
        score += count_invalid(range.to_string());
    }
    println!("Part1 Score: {}", score);

    Ok(())
}

fn count_invalid(input: String) -> i64 {
    let parts: Vec<&str> = input.split('-').collect();
    let start = parts[0].parse::<i64>().unwrap();
    let end = parts[1].parse::<i64>().unwrap();

    let mut count = 0;
    for number in start..=end {
        let mut num_str = number.to_string();
        let rest = num_str.split_off(num_str.len() / 2);
        if num_str == rest {
            count += number;
        }
    }
    return count;
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
            assert_eq!(count_invalid(input.into()), expected);
        }
    }
}
