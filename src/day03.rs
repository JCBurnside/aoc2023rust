use itertools::Itertools;
type Data = Vec<Number>;
enum Number {
    Valid(usize),
    Invalid(usize),
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_numeric()
}

fn parse(input: &str) -> Data {
    let mut numbers = Vec::new();
    let lines = input.trim().lines();
    let first = lines.clone().next().unwrap();
    let second = lines.clone().nth(1).unwrap();
    let mut first_scanner = first.char_indices();
    while let Some((idx, c)) = first_scanner.next() {
        if c.is_numeric() {
            let mut num = String::new();
            num.push(c);
            let mut valid = if idx == 0 {
                false
            } else {
                is_symbol(first.chars().nth(idx - 1).unwrap())
                || is_symbol(second.chars().nth(idx - 1).unwrap())
            };
            valid = valid || is_symbol(second.chars().nth(idx).unwrap());
            while let Some((idx, c)) = first_scanner.clone().next() {
                if !c.is_numeric() {
                    break;
                }
                let _ = first_scanner.next();
                num.push(c);
                valid = valid || is_symbol(second.chars().nth(idx).unwrap());
            }

            if let Some((idx, c)) = first_scanner.clone().next() {
                valid = valid || is_symbol(c) || is_symbol(second.chars().nth(idx).unwrap())
            }

            let number = num.parse().unwrap();
            numbers.push(if valid {
                Number::Valid(number)
            } else {
                Number::Invalid(number)
            });
        }
    }

    for (prev, curr, next) in lines.clone().tuple_windows() {
        let prev = |idx|{prev.chars().nth(idx).unwrap()};
        let next = |idx|{next.chars().nth(idx).unwrap()};
        let mut scanner = curr.char_indices();
        while let Some((idx, c)) = scanner.next() {
            if c.is_numeric() {
                let mut num = String::new();
                num.push(c);
                let mut valid = if idx == 0 {
                    false
                } else {
                    is_symbol(curr.chars().nth(idx - 1).unwrap())
                    || is_symbol(prev(idx - 1))
                    || is_symbol(next(idx - 1))
                }
                || is_symbol(prev(idx)) 
                || is_symbol(next(idx));

                while let Some((idx,c)) = scanner.clone().next() {
                    if !c.is_numeric() { break; }
                    let _ = scanner.next();
                    num.push(c);
                    valid = valid || is_symbol(prev(idx)) || is_symbol(next(idx));
                }

                if let Some((idx,c)) = scanner.clone().next() {
                    valid = valid || is_symbol(c) || is_symbol(prev(idx)) || is_symbol(next(idx));
                }

                let number = num.parse().unwrap();
                numbers.push(if valid {
                    Number::Valid(number)
                }else {
                    Number::Invalid(number)
                });
            }
        }
    }

    let lines = lines.rev();
    let last = lines.clone().next().unwrap();
    let prev = lines.clone().nth(1).unwrap();
    let mut last_scanner = last.char_indices();
    while let Some((idx, c)) = last_scanner.next() {
        if c.is_numeric() {
            let mut num = String::new();
            num.push(c);
            let mut valid = if idx == 0 {
                false
            } else {
                is_symbol(last.chars().nth(idx - 1).unwrap())
                || is_symbol(prev.chars().nth(idx - 1).unwrap())
            };
            valid = valid || is_symbol(prev.chars().nth(idx).unwrap());
            while let Some((idx, c)) = last_scanner.clone().next() {
                if !c.is_numeric() {
                    break;
                }
                let _ = last_scanner.next();
                num.push(c);
                valid = valid || is_symbol(prev.chars().nth(idx).unwrap());
            }

            if let Some((idx, c)) = last_scanner.clone().next() {
                valid = valid || is_symbol(c) || is_symbol(prev.chars().nth(idx).unwrap())
            }

            let number = num.parse().unwrap();
            numbers.push(if valid {
                Number::Valid(number)
            } else {
                Number::Invalid(number)
            });
        }
    }

    numbers
}

fn part1_impl(data :&Data) -> usize {
    data.iter().filter_map(|datum| match datum {
        Number::Valid(num) => Some(num),
        _ => None
    })
    .sum()
}

pub fn part1() {
    let data = parse(include_str!("../data/day03.txt"));
    println!("sum of all the valid part numbers is {}", part1_impl(&data));
}

pub fn part2() {
    println!("not yet implemented")
}

#[cfg(test)]
mod tests {
    const SAMPLE:&'static str = 
r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part1() {
        let data = super::parse(SAMPLE);
        assert_eq!(super::part1_impl(&data),4361)
    }
}
