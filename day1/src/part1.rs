use anyhow::{Context, Result};

#[tracing::instrument]
fn process(input: &str) -> Result<String> {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|ch| ch.to_digit(10));

            let first = it.next().expect("should be a number");
            match it.last() {
                Some(last) => first * 10 + last,
                None => first * 10 + first,
            }
        })
        .sum::<u32>();

    Ok(output.to_string())
}

#[tracing::instrument]
pub fn part1() -> Result<()> {
    let file = include_str!("../input1.txt");
    let result = process(file).context("process part 1")?;
    println!("Part 1: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r##"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "142");
    }
}
