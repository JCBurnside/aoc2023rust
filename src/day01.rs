type Data<'a> = Vec<&'a str>;
const RAW_DATA : &'static str = include_str!("../data/day01.txt");

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

pub fn part1() {
    let data : Data = RAW_DATA.trim().lines().map(str::trim).collect();
    println!("the sum of the calibration data is {}", part1_impl(&data))
}

fn part2_impl(data : &Data)-> usize {
    data.into_iter()
    .map(|line| {
        let chars = line.chars();
        let fst = (||{
            let mut chars = chars.clone();
            while let Some(c) = chars.next() {
                return match c {
                    c if c.is_numeric() => c.to_digit(10).unwrap(),
                    'o' if chars.clone().take(2).collect::<String>() == "ne" => 1,
                    't' if chars.clone().take(2).collect::<String>() == "wo" => 2,
                    't' if chars.clone().take(4).collect::<String>() == "hree" => 3,
                    'f' if chars.clone().take(3).collect::<String>() == "our" => 4,
                    'f' if chars.clone().take(3).collect::<String>() == "ive" => 5,
                    's' if chars.clone().take(2).collect::<String>() == "ix" => 6,
                    's' if chars.clone().take(4).collect::<String>() == "even" => 7,
                    'e' if chars.clone().take(4).collect::<String>() == "ight" => 8,
                    'n' if chars.clone().take(3).collect::<String>() == "ine" => 9,
                    _ => continue,
                }
            }
            unreachable!("the data is invalid")
        })();
        let lst = (||{
            let mut chars = chars.clone().rev();
            while let Some(c) = chars.next() {
                return match c {
                    c if c.is_numeric() => c.to_digit(10).unwrap(),
                    'e' if chars.clone().take(2).collect::<String>() == "no" => 1,//one
                    'o' if chars.clone().take(2).collect::<String>() == "wt" => 2,//two
                    'e' if chars.clone().take(4).collect::<String>() == "erht" => 3,//three
                    'r' if chars.clone().take(3).collect::<String>() == "uof" => 4,//four
                    'e' if chars.clone().take(3).collect::<String>() == "vif" => 5,//five
                    'x' if chars.clone().take(2).collect::<String>() == "is" => 6,//six
                    'n' if chars.clone().take(4).collect::<String>() == "eves" => 7,//seven
                    't' if chars.clone().take(4).collect::<String>() == "hgie" => 8,//eight
                    'e' if chars.clone().take(3).collect::<String>() == "nin" => 9,//nine
                    _ => continue,
                }
            }
            unreachable!("the data is invalid")
        })();
        format!("{fst}{lst}").parse::<usize>().unwrap()
    })
    .sum()
}

pub fn part2() {
    let data : Data = RAW_DATA.trim().lines().map(str::trim).collect();
    println!("the sum of the calibration data accounting for words is {}", part2_impl(&data))
}

#[cfg(test)]
mod tests {
    use super::Data;

    const SAMPLE_P1 : &'static str = 
r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    #[test]
    fn part1() {
        let data : Data = SAMPLE_P1.trim().lines().map(str::trim).collect();
        assert_eq!(super::part1_impl(&data),142)
    }
    const SAMPLE_P2 : &'static str = 
r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    #[test]
    fn part2() {
        let data : Data = SAMPLE_P2.trim().lines().map(str::trim).collect();
        assert_eq!(super::part2_impl(&data),281);
    }
}