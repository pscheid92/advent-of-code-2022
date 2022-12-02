extern crate aoc2022;
use aoc2022::*;

fn main() {
    let lines = read(1).expect("error reading input");
    let packs = convert_pack::<i32>(pack(lines)).expect("error parsing numbers");

    let solution = solve(&packs);

    println!("solution: {}", solution);
}

fn solve(packs: &Vec<Vec<i32>>) -> i32 {
    packs
        .iter()
        .map(|pack| pack.iter().sum::<i32>())
        .max()
        .unwrap()
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
        assert_eq!(solution, 24000);
    }
}
