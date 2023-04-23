#![allow(non_snake_case)]
use colored::*;
use std::fs;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

#[allow(unused_assignments)]
pub fn remove_postfix(file_name: String){ 
    
    let contents=fs::read_to_string(file_name.clone()).expect("Should have been able to read the file");
    let mut line_vec: Vec<String> = Vec::new();

    println!("Deleted postfix:");
    
    let re = Regex::new(r"\([a-zA-Z]\)").unwrap();
    
    line_vec = contents.lines().filter(|x| re.is_match(x)).map(|x| x.to_string()).collect();

    find_low_let(line_vec);
    
    let re = Regex::new(r"\s\([a-zA-Z]\)").unwrap();
    let mut contents = re.replace_all(&contents, "").to_string();
    let re = Regex::new(r"\([a-zA-Z]\) ").unwrap();
    contents = re.replace_all(&contents, "").to_string();
    contents = contents.lines().map(|x| x.trim_end_matches(',').to_string()).collect::<Vec<String>>().join("\n");
    sort_and_save(contents, file_name);
}

fn sort_and_save(contents: String, file_name: String)
{
    let mut buffer = File::create(file_name).unwrap();
    let mut splits_vec: Vec<String> = Vec::new();
    let mut first = true;
    let mut first_line= "";

    for line in contents.lines()
    {
        if first
        {
            first_line=line;
            first=false;
            continue;
        }
        splits_vec.push(line.to_string());
    }
    splits_vec.sort();
    splits_vec.push(first_line.to_string());
    splits_vec.rotate_right(1);
    
    for line in splits_vec.iter()
    {
        buffer.write_all((line.to_string()+"\n").as_bytes()).unwrap();
    }
}


fn find_low_let(line_vec: Vec<String>) {
    let re = Regex::new(r"\([a-z]\)").unwrap();
    for line in line_vec.iter()
    {
        let splits: Vec<&str> = line.split("\",").collect();
        let name_with_postfix: String = splits[0].trim_matches('"').to_string();
        if re.is_match(&name_with_postfix)
        {
            postf_select_low(&name_with_postfix);
            println!("{}", "The prefix is written with a small letter!".to_string().red());
        }
        let re =Regex::new(r"\([A-Z]\)").unwrap();
        if re.is_match(&name_with_postfix)
        {
            postf_select_big(&name_with_postfix);
        }
    } 
}

fn postf_select_low(line: &String)
{
    let re = Regex::new(r"\([a-z]\)").unwrap();
    for words in line.split_whitespace()
    {
        if re.is_match(&words)
        {
            print!("{}", words.red());
            continue;
        }
        print!("{words} ");
    }
    println!();
}

fn postf_select_big(line: &String){
    let re = Regex::new(r"\([A-Z]\)").unwrap();
    for words in line.split_whitespace()
    {
        if re.is_match(&words)
        {
            print!("{}", words.blue());
            continue;
        }
        print!("{words} ");
    }
    println!();
}