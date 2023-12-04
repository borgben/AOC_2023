use std::env;
use std::fs;
use std::io::{self,BufRead};
use regex::Regex;

fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

struct Game{
    id:u32, 
    largest_red: u64,
    largest_green: u64,
    largest_blue: u64,
}

fn extract_games(game_strings:Vec<String>) -> Vec<Game>{
    let mut game_vec = Vec::<Game>::new();
    let num_re = Regex::new(r"\d+").unwrap();
    let red_result_re = Regex::new(r" (\d+) red").unwrap();
    let blue_result_re = Regex::new(r" (\d+) blue").unwrap();
    let green_result_re = Regex::new(r" (\d+) green").unwrap();
    for game_string in game_strings{
        let game_id = num_re.find_iter(&game_string).map(|mat| mat.as_str()).collect::<Vec<&str>>()[0].parse::<u32>().unwrap();
        let game_results = flatten::<&str>((game_string.split(":").collect::<Vec<&str>>()[1]).split(";").collect::<Vec<&str>>().iter().map(|s| s.split(",").collect::<Vec<&str>>()).collect());
        let red_results  = game_results.clone().iter().filter(|s| (red_result_re.is_match(s))).map(|s| num_re.find_iter(s).map(|mat| mat.as_str()).collect::<Vec<&str>>()[0].parse::<u64>().unwrap()).fold(std::u64::MIN,|a,b| a.max(b));
        let blue_results = game_results.clone().iter().filter(|s| (blue_result_re.is_match(s))).map(|s| num_re.find_iter(s).map(|mat| mat.as_str()).collect::<Vec<&str>>()[0].parse::<u64>().unwrap()).fold(std::u64::MIN,|a,b| a.max(b));
        let green_results= game_results.clone().iter().filter(|s| (green_result_re.is_match(s))).map(|s| num_re.find_iter(s).map(|mat| mat.as_str()).collect::<Vec<&str>>()[0].parse::<u64>().unwrap()).fold(std::u64::MIN,|a,b| a.max(b));

        let game = Game{
            id:game_id,
            largest_red:red_results,
            largest_green:green_results,
            largest_blue:blue_results,
        };
        game_vec.push(game);
    }
    return game_vec
}

fn main(){
    // let args: Vec<String> = env::args().collect();
    // println!("In file {}", args[1]);
    env::set_var("RUST_BACKTRACE", "1");
    let file_ref = fs::File::open(&("aoc_02_input.txt".to_string()))
        .expect("Should have been able to read the file");

    let file_buffer = io::BufReader::new(file_ref);

    let contents: Vec<String> = file_buffer.lines().map(|l| l.expect("Invalid String")).collect();

    let game_results = extract_games(contents);
    println!("{}",game_results.iter().filter(|game| !((game.largest_red > 12) | (game.largest_blue > 14) | (game.largest_green > 13))).fold(std::u32::MIN,|a,b| a + b.id).to_string());
    println!("{}",game_results.iter().map(|game| game.largest_blue*game.largest_green*game.largest_red).fold(std::u64::MIN,|a,b| a + b).to_string());
}