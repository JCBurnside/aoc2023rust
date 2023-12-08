type Data = Vec<Game>;

#[cfg_attr(test, derive(PartialEq,Eq,Debug))]
enum Cube {
    Red,
    Green,
    Blue,
}


#[cfg_attr(test, derive(PartialEq,Eq,Debug))]
struct Game {
    game_id : usize,
    draws : Vec<Vec<(Cube,usize)>>
}

fn parse_draw(draw:&str) -> Vec<(Cube,usize)> {
    draw.trim().split(',')
    .map(|count| {
        let Some((count, color)) = count.trim().split_once(' ') else { unreachable!("data in bad format.") };
        let color = match color {
            "red" => Cube::Red,
            "green" => Cube::Green,
            "blue" => Cube::Blue,
            _ => unreachable!(),
        };
        (color,count.parse().unwrap())
    })
    .collect()
}

fn parse(input : &str) -> Data {
    input.trim().lines()
    .map(|line| {
        let Some((game_id, data)) = line.split_once(':') else { unreachable!("data in bad format. check id of {line}") };
        let game_id = game_id[4..].trim().parse().unwrap();
        
        Game {
            game_id,
            draws : data.split(';').map(parse_draw).collect()
        }
    })
    .collect()
}

pub fn part1() {}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::Game;
    use super::Cube::*;
    const SAMPLE : &'static str = 
r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    #[test]
    fn parse() {
        let data = super::parse(SAMPLE);
        assert_eq!(
            data,
            [
                Game { game_id : 1, draws: vec![vec![(Blue,3),(Red,4)],vec![(Red, 1),(Green, 2),(Blue,6)],vec![(Green,2)]]},
                Game { game_id : 2, draws: vec![vec![(Blue,1),(Green,2)],vec![(Green, 3),(Blue, 4), (Red, 1)],vec![(Green, 1), (Blue,1)]]},
                Game { game_id : 3, draws: vec![vec![(Green,8),(Blue,6),(Red,20)],vec![(Blue, 5),(Red, 4), (Green, 13)],vec![(Green, 5), (Red, 1)]]},
                Game { game_id : 4, draws: vec![vec![(Green,1),(Red,3),(Blue,6)],vec![(Green, 3),(Red, 6)],vec![(Green, 3), (Blue,15), (Red,14)]]},
                Game { game_id : 5, draws: vec![vec![(Red, 6), (Blue, 1), (Green, 3)],vec![(Blue,2), (Red,1),(Green, 2)]]},
            ]
        );
    }
}