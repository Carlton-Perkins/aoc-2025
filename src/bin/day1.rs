use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day1-part1.txt");
    let result = solve(input)?;
    println!("Result: {}", result);
    Ok(())
}

fn solve(input: &str) -> Result<i32> {
    let lines: Vec<_> = input.lines().collect();
    let mut pos = 50;
    let mut score = 0;
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        match (chars[0], &chars[1..].iter().collect::<String>()) {
            ('L', num) => {
                let n: i32 = num.parse()?;
                pos -= n;
            }
            ('R', num) => {
                let n: i32 = num.parse()?;
                pos += n;
            }
            _ => anyhow::bail!("invalid input"),
        }

        pos = pos % 100;
        if pos == 0 {
            score += 1;
        }
    }
    Ok(score)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let input = include_str!("../../inputs/day1-example.txt");
        assert_eq!(solve(input).unwrap(), 3);
    }
}
