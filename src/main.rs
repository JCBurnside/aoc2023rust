use std::io::Write;

const DAYS: [[fn(); 2]; 0] = [];
fn main() {
    let mut input = String::new();
    loop {
        print!("please enter what day to run or q to exit ");
        std::io::stdout().lock().flush().unwrap();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "q" {
            break;
        }
        match input.trim().parse::<usize>() {
            Ok(day) if day <= DAYS.len() && day != 0 => loop {
                print!("please enter part 1 or 2 ");
                std::io::stdout().lock().flush().unwrap();
                input.clear();
                std::io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<usize>() {
                    Ok(part) if part != 0 && part <= 2 => {
                        DAYS[day - 1][part - 1]();
                        break;
                    }
                    _ => println!("invalid part selected"),
                }
            },
            _ => println!("invalid day selected."),
        }
    }
    println!("Good Bye!");
}
