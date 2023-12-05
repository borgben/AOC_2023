use std::env;
use std::fs;
use std::io::{self,BufRead};
use itertools::Itertools;
// use regex::Regex;

fn gear_ratio(contents:Vec<Vec<char>>) -> u64 {
    let mut i:usize = 0;
    let mut j:usize = 0;
    let mut agg:u64 = 0;

    while i < contents.len(){
        while j < contents[0].len(){
            let current_char:char = contents[i][j];

            if current_char == '*'{
                let mut gear_ratio_vector:Vec<u64>=Vec::new();
                let mut current_value = String::new();
                for x in -1..2{
                    for y in -1..2{
                        let check_i:i32 = <usize as TryInto<i32>>::try_into(i).unwrap()+x;
                        let check_j:i32 = <usize as TryInto<i32>>::try_into(j).unwrap()+y;
                        if (check_i<contents.len().try_into().unwrap())&(check_j<contents[0].len().try_into().unwrap()){
                            if (check_i>=0)&(check_j>=0){
                                let check_i_usize:usize = check_i.try_into().unwrap();
                                let check_j_usize:usize = check_j.try_into().unwrap();
                                if contents[check_i_usize][check_j_usize].is_digit(10){
                                    let mut z = check_j_usize;
                                    while contents[check_i_usize][z].is_digit(10)&(z>0){
                                        z -=1;
                                    }

                                    if (!(z<(contents[check_i_usize].len())&&contents[check_i_usize][z].is_digit(10))){
                                        z +=1;
                                    }

                                    while (z<(contents[check_i_usize].len()))&&contents[check_i_usize][z].is_digit(10){
                                        current_value.push(contents[check_i_usize][z]);
                                        z+=1;
                                    }

                                    gear_ratio_vector.push(current_value.parse().unwrap());
                                    current_value = String::new();
                                }
                            }
                        } 
                    }
                }

                
                if gear_ratio_vector.iter().unique().collect::<Vec<&u64>>().len() == 2{
                    agg += gear_ratio_vector.iter().unique().fold(1,|a,b| a*(*b));
                }
            }
            j +=1;
        }
        j = 0;
        i+=1;
    }
    return agg;
}

fn extract_values(contents:Vec<Vec<char>>) -> u64 {
    let mut i:usize = 0;
    let mut j:usize= 0;
    let mut agg:u64  = 0;

    while i < contents.len(){

        let mut current_value:String = String::new();
        let mut is_valid:bool = false;

        while j < contents[0].len().try_into().unwrap(){
            
            let current_char:char = contents[i][j];

            if current_char.is_digit(10){
                current_value.push(current_char);

                for x in -1..2{
                    for y in -1..2{
                        let check_i:i32 = <usize as TryInto<i32>>::try_into(i).unwrap()+x;
                        let check_j:i32 = <usize as TryInto<i32>>::try_into(j).unwrap()+y;
                        
                        if (check_i<contents.len().try_into().unwrap())&(check_j<contents[0].len().try_into().unwrap()){
                            if (check_i>=0)&(check_j>=0){
                                let check_i_usize:usize = check_i.try_into().unwrap();
                                let check_j_usize:usize = check_j.try_into().unwrap();
                                if ((contents[check_i_usize][check_j_usize]).is_ascii_punctuation())&(contents[check_i_usize][check_j_usize]!='.'){
                                    is_valid = true;
                                }
                            }

                        } 
                    }
                }
            }

            if !current_char.is_digit(10){
                if (current_value.len() > 0)&(is_valid){
                    agg += current_value.parse::<u64>().unwrap();
                }
                is_valid = false;
                current_value = String::new();
            }
            j += 1;
        }

        if (current_value.len() > 0)&(is_valid){
            agg += current_value.parse::<u64>().unwrap();
        }
        is_valid = false;
        current_value = String::new();
        j = 0;
        i += 1;
    }
    return agg;
}

fn main(){
    // let args: Vec<String> = env::args().collect();
    // println!("In file {}", args[1]);
    env::set_var("RUST_BACKTRACE", "1");
    let file_ref = fs::File::open(&("aoc_03_input.txt".to_string()))
        .expect("Should have been able to read the file");

    let file_buffer = io::BufReader::new(file_ref);

    let contents: Vec<Vec<char>> = file_buffer.lines().map(|l| l.expect("Invalid String").chars().collect()).collect();

    let game_results = extract_values(contents.clone());
    let gear_ratio:u64 = gear_ratio(contents.clone());
    println!("{}",game_results);
    println!("{}",gear_ratio);
}