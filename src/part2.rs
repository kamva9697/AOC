use std::fs; 

const INPUT_FILE: &str = "C:\\Users\\cna\\myRust\\AOC\\day1\\src\\input.txt";

pub fn part2() -> std::io::Result<()> {
    let file = fs::read_to_string(INPUT_FILE)?;
    let mut depths = Vec::<i32>::new();
    let mut prev = 0;
    let mut sum = 0;

    for line in file.lines() {
        depths.push(line.parse::<i32>().unwrap());
    }

    let mut max_sum;
    for window in depths.windows(3).skip(3) {
        max_sum = window.iter().sum::<i32>(); 
        
        if prev < max_sum {
            sum += 1;
        }
        prev = max_sum;
    }
    println!("part 2: {}", sum);
    Ok(())
}