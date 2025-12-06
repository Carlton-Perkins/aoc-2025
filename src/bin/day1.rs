use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../inputs/day1-part1.txt");
    let result = solve1(input)?;
    println!("Result: {}", result);

    let result2 = solve2(input)?;
    println!("Result 2: {}", result2);
    Ok(())
}

fn solve1(input: &str) -> Result<i32> {
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

fn solve2(input: &str) -> Result<i32> {
    let lines: Vec<_> = input.lines().collect();
    let mut pos = 50;
    let mut score = 0;
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        let mut n = match (chars[0], &chars[1..].iter().collect::<String>()) {
            ('L', num) => {
                let n: i32 = num.parse()?;
                -n
            }
            ('R', num) => {
                let n: i32 = num.parse()?;
                n
            }
            _ => anyhow::bail!("invalid input"),
        };

        while n != 0 {
            if n > 0 {
                pos += 1;
                n -= 1;
            } else {
                pos -= 1;
                n += 1;
            }
            pos = (pos + 100) % 100;
            if pos == 0 {
                score += 1;
            }
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
        assert_eq!(solve1(input).unwrap(), 3);
    }

    #[test]
    fn example2() {
        let input = include_str!("../../inputs/day1-example.txt");
        assert_eq!(solve2(input).unwrap(), 6);
    }
}
