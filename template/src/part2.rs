use anyhow::{Context, Result};

#[tracing::instrument]
fn process(_input: &'static str) -> Result<String> {
    Ok("".to_string())
}

#[tracing::instrument]
pub fn part2(input: &'static str) -> Result<String> {
    process(input).context("process part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r##""##;
        let result = process(input).unwrap();
        assert_eq!(result, "42");
    }
}
