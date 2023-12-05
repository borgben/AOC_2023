use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self,BufRead};
use regex::Regex;
// use itertools::Itertools;

#[derive(Clone)]
struct Card{
    id:u64,
    points:u64,
    count:u64,
}

fn count_cards(mut card_vec:Vec<Card>) -> u64{
    let og_len = card_vec.len();
    for x in 0..og_len.clone(){
        let start:usize = <u64 as TryInto<usize>>::try_into(card_vec[x].id).unwrap();
        let mut end:usize   = (card_vec[x].id + card_vec[x].points).try_into().unwrap();
        if end >= og_len{
            end = og_len.clone();
        }
        for i in start..end{
            card_vec[i].count = card_vec[i].count + (1*card_vec[x].count);
        }
    }
    return card_vec.iter().fold(0,|a,b| a+b.count);
}

fn card_recursion(contents:Vec<String>) -> u64{
    let mut card_vec:Vec<Card> = Vec::new();
    for row in contents{
        let mut number_map:HashMap<u64,u64> = HashMap::new();
        let num_re = Regex::new(r"\d+").unwrap();
        let card_id = num_re.find(row.clone().split(":").collect::<Vec<&str>>()[0]).unwrap().as_str().parse::<u64>().unwrap();
        let card = String::from(row.clone().split(":").collect::<Vec<&str>>()[1]);
        for num in num_re.find_iter(card.clone().split("|").collect::<Vec<&str>>()[0]).map(|mat| mat.as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>(){
            number_map.insert(num,0);
        }
        for num in num_re.find_iter(card.clone().split("|").collect::<Vec<&str>>()[1]).map(|mat| mat.as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>(){
            if number_map.contains_key(&num){
                number_map.insert(num,number_map[&num]+1);
            }
        }
        card_vec.push(Card{id:card_id,points:number_map.iter().fold(0,|a,b| a+(b.1)),count:1});
    }

    return count_cards(card_vec);

}

fn get_points(contents:Vec<String>) -> u64{
    let mut agg:u64 = 0;
    for row in contents{
        let mut number_map:HashMap<u64,u64> = HashMap::new();
        let num_re = Regex::new(r"\d+").unwrap();
        let card = String::from(row.split(":").collect::<Vec<&str>>()[1]);
        for num in num_re.find_iter(card.clone().split("|").collect::<Vec<&str>>()[0]).map(|mat| mat.as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>(){
            number_map.insert(num,0);
        }
        for num in num_re.find_iter(card.clone().split("|").collect::<Vec<&str>>()[1]).map(|mat| mat.as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>(){
            if number_map.contains_key(&num){
                number_map.insert(num,number_map[&num]+1);
            }
        }
        let matches = number_map.iter().fold(0,|a,b| a+(b.1));
        if matches > 0{
            agg += 2_u64.pow( (matches - 1).try_into().unwrap());
        }
        
    }
    return agg;
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // println!("In file {}", args[1]);
    env::set_var("RUST_BACKTRACE", "1");
    let file_ref = fs::File::open(&("aoc_04_input.txt".to_string()))
        .expect("Should have been able to read the file");

    let file_buffer = io::BufReader::new(file_ref);

    let contents: Vec<String> = file_buffer.lines().map(|l| l.expect("Invalid String")).collect();
    println!("{}",get_points(contents.clone()).to_string());
    println!("{}",card_recursion(contents.clone()).to_string());
}
