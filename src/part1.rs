use std::fs; 

const INPUT_FILE: &str = "C:\\Users\\cna\\myRust\\AOC\\day1\\src\\input.txt";

pub fn part1() -> std::io::Result<()> {
    let file = fs::read_to_string(INPUT_FILE)?;
    let mut prev: i32 = 0;
    let mut sum: i32 = 0;

    for line in file.lines() {
        let depth = line.parse::<i32>().unwrap();
        if prev < depth {
            sum += 1;
        }
        prev = depth;
    }
    println!("part 1: {}", sum - 1);
    Ok(())
}