use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let games = parse(&input);
    println!("sum = {:?}", sum(games, 12, 14, 13))
}

fn parse(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for (_, s) in input.split('\n').map(|s| s.trim()).enumerate() {
        match Game::parse(s) {
            Some(game) =>games.push(game),
            None => (),
        }      
    }
    games
}

fn sum(games: Vec<Game>, red:u32, blue:u32, green:u32)->(u32,u32) {
    let mut sum:u32 = 0;
    let mut pwr_sum:u32 = 0;
    for g in games.iter() {

        if g.red_max == None || g.blue_max == None || g.green_max == None {
            println!("none case");
            continue;
        }else if red < g.red_max.unwrap() || blue < g.blue_max.unwrap() || green < g.green_max.unwrap() {
            //println!("red({}/{}), blue({}/{}), green({}/{})", red, g.red_max.unwrap(), blue, g.blue_max.unwrap(), green, g.green_max.unwrap());
            pwr_sum += g.red_max.unwrap() * g.blue_max.unwrap() * g.green_max.unwrap();
            continue;
        }

        //println!("sum: {}", sum);
        // if g.red_max <= red
        println!("{:?}", g);
        sum += g.id;

        pwr_sum += g.red_max.unwrap() * g.blue_max.unwrap() * g.green_max.unwrap();
    }
    (sum,pwr_sum)
}

#[derive(Debug)]
struct Game {
    id: u32,
    red_max: Option<u32>,
    blue_max: Option<u32>,
    green_max: Option<u32>,
}

// ["Game 1", "3 blue, 4 red", "1 red, 2 green, 6 blue", "2 green"]
// ["Game 2", "1 blue, 2 green", "3 green, 4 blue, 1 red", "1 green, 1 blue"]
// ["Game 3", "8 green, 6 blue, 20 red", "5 blue, 4 red, 13 green", "5 green, 1 red"]
// ["Game 4", "1 green, 3 red, 6 blue", "3 green, 6 red", "3 green, 15 blue, 14 red"]
// ["Game 5", "6 red, 1 blue, 3 green", "2 blue, 1 red, 2 green"]

impl Game {
    fn parse(input:&str) -> Option<Game> {
        println!("{}", input);
        // let game_vec:Vec<&str> = input.split(&[':', ';'][..]).map(|s| s.trim()).collect();
        
        // println!("{:?}", game_vec);

        let re_game = Regex::new(r"Game (?<id>\d+)").unwrap(); //r"Game (?<id>\d)"
        let re_red = Regex::new(r"(?<cnt>\d+) (?<color>red)").unwrap(); //r"Game (?<id>\d)"
        let re_blue = Regex::new(r"(?<cnt>\d+) (?<color>blue)").unwrap(); //r"Game (?<id>\d)"
        let re_green = Regex::new(r"(?<cnt>\d+) (?<color>green)").unwrap(); //r"Game (?<id>\d)"

        let Some(caps_game) = re_game.captures(input) else { return None };
        // println!("{:?}",&caps_game["id"].parse::<u32>().unwrap());

        let id = caps_game["id"].parse::<u32>().unwrap();

        let mut red : Option<u32>= None;
        let mut blue : Option<u32> = None;
        let mut green : Option<u32> = None;

        for caps in re_red.captures_iter(input) {
//            let color = &caps["color"];
            let cnt = caps["cnt"].parse::<u32>().unwrap();

            // println!("Color:{}, Count:{}",&caps["color"], &caps["cnt"].parse::<u32>().unwrap());

            if red == None { red = Some(cnt);}
            else if red.unwrap() < cnt {red = Some(cnt);}
        }

        for caps in re_blue.captures_iter(input) {
            let cnt = caps["cnt"].parse::<u32>().unwrap();
            if blue == None { blue = Some(cnt);}
            else if blue.unwrap() < cnt {blue = Some(cnt);}
        }
            

        for caps in re_green.captures_iter(input) {
            let cnt = caps["cnt"].parse::<u32>().unwrap();
            if green == None { green = Some(cnt);}
            else if green.unwrap() < cnt {green = Some(cnt);}
        }

        // for s in input.split([':',';'])

        let ret = Game {
            id,
            red_max: red,
            blue_max: blue,
            green_max: green,
        };

        println!("{:?}",ret);
        Some(ret)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn check_game_no(){
        
    //     parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    //     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    //     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    //     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    //     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
    //     assert_eq!(1,-1);
    // }

    // #[test]
    // fn calc() {
    //     let games = parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    //     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    //     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    //     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    //     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    //     Game 6: 2 red, 5 green, 1 blue; 3 blue, 5 green; 8 blue, 13 green, 2 red; 9 green, 3 blue; 12 green, 13 blue; 3 green, 3 blue, 1 red");

    //     let s = sum(games,12,13,14);
    //     assert_eq!(s,8);
    // }
}
