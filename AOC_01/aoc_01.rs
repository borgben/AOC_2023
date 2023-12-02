use std::env;
use std::fs;
use std::io::{self,BufRead};

fn string_with_digits_as_digits(trg_str:String) -> String {
    let replace_list: Vec<(String,String)> = vec![("zero".to_string(),"0".to_string()),("one".to_string(),"1".to_string()),("two".to_string(),"2".to_string()),("three".to_string(),"3".to_string()),("four".to_string(),"4".to_string()),("five".to_string(),"5".to_string()),("six".to_string(),"6".to_string()),("seven".to_string(),"7".to_string()),("eight".to_string(),"8".to_string()),("nine".to_string(),"9".to_string())];
    let mut build_string = String::from("");
    for chr in trg_str.chars(){
        build_string.push(chr);
        for tuple in &replace_list{
            build_string = build_string.replace(&tuple.0,&tuple.1);
        }
    }
    return build_string
}

fn string_to_two_digit(trg_str:&String) -> i32{
    let trg_str_two:String  = string_with_digits_as_digits(trg_str.to_string());
    let mut result_string: String =  String::from("");
    let char_vec: Vec<char> = (*trg_str_two).chars().filter(|chr| (chr).is_digit(10)).collect();
    result_string.push(*(char_vec.first().unwrap()));
    result_string.push(*(char_vec.last().unwrap()));
    return result_string.parse::<i32>().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("In file {}", args[1]);

    let file_ref = fs::File::open(&args[1])
        .expect("Should have been able to read the file");

    let file_buffer = io::BufReader::new(file_ref);

    let contents: Vec<String>= file_buffer.lines().map(|l| l.expect("Invalid String")).collect();
    println!("{}",contents.iter().map(|s| string_to_two_digit(&s)).reduce(|a,b| a+b).unwrap().to_string())
}