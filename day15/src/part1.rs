use anyhow::{Context, Result};
use tracing::info;

#[tracing::instrument(skip(input))]
fn process(input: &'static str) -> Result<String> {
    info!("processing input");

    let result = input
        .split(",")
        .map(|inst| {
            inst.chars()
                .filter(|c| c != &'\n')
                .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
        })
        .sum::<usize>();

    Ok(result.to_string())
}

#[tracing::instrument(skip(input))]
pub fn part1(input: &'static str) -> Result<String> {
    info!("part 1");

    process(input).context("process part 1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_rn1() {
        let input = r##"rn=1"##;
        let result = process(input).unwrap();
        assert_eq!(result, "30");
    }

    #[test_log::test]
    fn it_works() {
        let input = r##"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"##;
        let result = process(input).unwrap();
        assert_eq!(result, "1320");
    }
}
