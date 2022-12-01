extern crate aoc2022;
use aoc2022::*;

fn main() {
    let lines = read(1).expect("error reading input");
    let packs = convert_pack::<i32>(pack(lines)).expect("error parsing numbers");

    println!("part 1: {}", solve_part_one(&packs));
    println!("part 2: {}", solve_part_two(&packs));
}

fn solve_part_one(packs: &Vec<Vec<i32>>) -> i32 {
    packs
        .iter()
        .map(|pack| pack.iter().sum::<i32>())
        .max()
        .unwrap()
}

fn solve_part_two(packs: &Vec<Vec<i32>>) -> i32 {
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
    fn test_part_one() {
        let input = example_input(); 
        let solution = solve_part_one(&input);
        assert_eq!(solution, 24000);
    }

    #[test]
    fn test_part_two() {
        let input = example_input();
        let solution = solve_part_two(&input);
        assert_eq!(solution, 45000);
    }
}
