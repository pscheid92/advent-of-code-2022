use std::collections::HashSet;

use anyhow::{anyhow, Context};

use aoc2022::read;

fn main() -> anyhow::Result<()> {
    let lines = read(6).context("error reading input")?;
    let result = find_marker(&lines[0]).context("error finding marker")?;
    println!("result: {}", result);

    Ok(())
}

const MARKER_LENGTH: usize = 4;

fn find_marker(buffer: &str) -> anyhow::Result<usize> {
    if buffer.len() < MARKER_LENGTH {
        return Err(anyhow!("not enough characters in buffer"));
    }

    let position = buffer
        .as_bytes()
        .windows(MARKER_LENGTH)
        .position(|w| w.iter().collect::<HashSet<&u8>>().len() == MARKER_LENGTH)
        .map(|i| i + MARKER_LENGTH)
        .context("was not able to find marker in stream")?;

    Ok(position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cases() -> anyhow::Result<()> {
        struct TestCase {
            input: String,
            expected: anyhow::Result<usize>,
        }

        let cases = vec![
            TestCase {input: "abc".to_string(), expected: Err(anyhow!("error"))},
            TestCase {input: "aaaaaaaaaaaaa".to_string(), expected: Err(anyhow!("error"))},
            TestCase {input: "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), expected: Ok(7)},
            TestCase {input: "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), expected: Ok(5)},
            TestCase {input: "nppdvjthqldpwncqszvftbrmjlhg".to_string(), expected: Ok(6)},
            TestCase {input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), expected: Ok(10)},
            TestCase {input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), expected: Ok(11)},
        ];

        for tc in cases.iter() {
            match (find_marker(&tc.input), &tc.expected) {
                (Err(_), Err(_)) => continue,
                (Ok(g), Ok(e)) => assert_eq!(g, *e),
                (Ok(_), Err(_)) => panic!("expected error, got ok"),
                (Err(_), Ok(_)) => panic!("expected ok, got error"),
            }
        }

        Ok(())
    }
}
