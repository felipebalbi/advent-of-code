use anyhow::{Context, Result};

#[tracing::instrument]
fn process_line(input: &str) -> u32 {
    let mut it = (0..input.len()).filter_map(|index| {
        let trimmed_line = &input[index..];
        let result = if trimmed_line.starts_with("one") {
            '1'
        } else if trimmed_line.starts_with("two") {
            '2'
        } else if trimmed_line.starts_with("three") {
            '3'
        } else if trimmed_line.starts_with("four") {
            '4'
        } else if trimmed_line.starts_with("five") {
            '5'
        } else if trimmed_line.starts_with("six") {
            '6'
        } else if trimmed_line.starts_with("seven") {
            '7'
        } else if trimmed_line.starts_with("eight") {
            '8'
        } else if trimmed_line.starts_with("nine") {
            '9'
        } else {
            trimmed_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });

    let first = it.next().expect("should be a number");

    match it.last() {
        Some(last) => first * 10 + last,
        None => first * 10 + first,
    }
}

#[tracing::instrument]
fn process(input: &str) -> Result<String> {
    let output = input.lines().map(process_line).sum::<u32>();

    Ok(output.to_string())
}

#[tracing::instrument]
pub fn part2() -> Result<()> {
    let file = include_str!("../input2.txt");
    let result = process(file).context("process part 2")?;
    println!("Part 2: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r##"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"##;
        let result = process(input).unwrap();
        assert_eq!(result, "281");
    }
}
