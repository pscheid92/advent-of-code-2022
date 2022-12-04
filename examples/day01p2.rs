extern crate aoc2022;
use aoc2022::*;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let lines = read(1).context("error reading input")?;
    let packs = convert_pack::<i32>(pack(lines)).context("error parsing numbers")?;
    let solution = solve(&packs);
    println!("solution: {}", solution);
    Ok(())
}

fn solve(packs: &Vec<Vec<i32>>) -> i32 {
    let mut sums: Vec<i32> = packs
        .iter()
        .map(|pack| pack.iter().sum::<i32>())
        .collect();

    sums.sort_unstable();
    sums.reverse();

    sums.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    
    fn example_input() -> Vec<Vec<i32>> {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ] 
    }

    #[test]
    fn test_solve() {
        let input = example_input();
        let solution = solve(&input);
        assert_eq!(solution, 45000);
    }
}
