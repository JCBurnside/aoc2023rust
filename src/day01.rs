type Data<'a> = Vec<&'a str>;

fn part1_impl(data : &Data) -> usize {
    data.into_iter()
    .map(|line| {
        let chars = line.chars();
        let fst = chars.clone().find(|c| c.is_numeric()).unwrap();
        let lst = chars.rev().find(|c| c.is_numeric()).unwrap();
        format!("{fst}{lst}").parse::<usize>().unwrap()
    })
    .sum()
}

const RAW_DATA : &'static str = include_str!("../data/day01.txt");
pub fn part1() {
    let data : Data = RAW_DATA.trim().lines().map(str::trim).collect();
    println!("the sum of the calibration data is {}", part1_impl(&data))
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::Data;

    const SAMPLE : &'static str = 
r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    #[test]
    fn part1() {
        let data : Data = SAMPLE.trim().lines().map(str::trim).collect();
        assert_eq!(super::part1_impl(&data),142)
    }
}