#![allow(non_snake_case)]
use colored::*;
use std::io::prelude::*;
use std::fs::File;

pub fn remove_postfix(contents: String){ 
    
    let mut fin_string: String = "".to_string();

    println!("Deleted postfix:");
    for line in contents.lines() {
        
        let splits: Vec<&str> = line.split("\",").collect();
        let mut split_vec: Vec<String> = splits.iter().map(|x| x.trim_matches('"').to_string()).collect();

        split_vec[0]= match lower(&mut split_vec[0])
        {
            Ok(spl) => spl,
            Err(err) => {println!("{}", err.to_string().red()); std::process::exit(1)}
        };

        for i in 0..(split_vec.len()-2)
        {
            fin_string = fin_string + "\""+&split_vec[i].clone()+"\",";
        }
        fin_string = fin_string + "\""+&split_vec[split_vec.len()-2].to_string()+"\",\n";
    }

    sort_and_save(fin_string);
}

fn sort_and_save(contents: String)
{
    let mut buffer = File::create("union.csv").unwrap();
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


fn lower(splits_vec: &mut String) -> Result<String, &str>
{
    if splits_vec.contains("(s)") || splits_vec.contains("(p)")
    {
        postf_select(&splits_vec);
        return Err("The prefix is written with a small letter");
    }
    
    if splits_vec.contains("(S)") || splits_vec.contains("(P)")
    {
        postf_select(&splits_vec);
        *splits_vec=splits_vec.replace("(S)", "");
        *splits_vec=splits_vec.replace("(P)", "");
        *splits_vec = splits_vec.trim_matches(' ').to_string();
    }

    return Ok(splits_vec.to_string());
}

fn postf_select(line: &String)
{
    for words in line.split_whitespace()
    {
        if words.to_string().contains("(s)") || words.to_string().contains("(p)")
        {
            print!("{}", words.red());
            continue;
        } else if words.to_string().contains("(S)") || words.to_string().contains("(P)") {
            print!("{}", words.blue());
            continue;
        }
        print!("{words} ");
    }
    println!();
}