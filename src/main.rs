mod part1;
mod part2;

fn main() -> std::io::Result<()> {
    part1::part1()?;
    part2::part2()?;
    Ok(())
}