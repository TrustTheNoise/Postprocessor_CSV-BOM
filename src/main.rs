#![allow(non_snake_case)]
use std::{fs, vec, io};
use std::path::Path;
use std::env;
use std::io::prelude::*;
use std::fs::File;

mod remove_postf;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let binding3 = "example2.csv".to_string();
        let binding = "smart speaker v.1 bom.csv".to_string();
        let binding2 = "smart speaker v.2 bom.csv".to_string();
        let files=Files
        {
            file_path: vec![&binding3, &binding2, &binding],
            flag: false,
        };
        run (files);
    }
}

// If you see comments on obvious things, that's fine

struct Files<'a>
{
    file_path: Vec<&'a String>,
    flag: bool
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut files = Files{
        file_path: vec![],
        flag: false,
    };

    for i in 1..args.len()-1
    {
        files.file_path.push(&args[i]);
    }


    if args.contains(&"-f".to_string())
    {
        files.flag = true;
    }else
    {
        files.file_path.push(&args[args.len()-1]);
    }
    run(files);
}

fn run(files: Files) {
    let mut contents: Vec<String> = Vec::new();

    for i in 0..files.file_path.len()
    {
        contents.push(fs::read_to_string(files.file_path[i])
                        .expect("Should have been able to read the file"));
    }
    let mut name_of_file = String::new();
    
    println!("\nWrite name of the union file:");
    io::stdin()
        .read_line(&mut name_of_file)
        .expect("Failed to read line");
    let mut fin_name = name_of_file.trim().to_string();
    let mut file_num=1;

    while Path::new(&(fin_name.clone() + ".csv")).exists()
    {
        fin_name = name_of_file.trim().to_string() + &file_num.to_string();
        file_num += 1;
    }
    fin_name = fin_name + ".csv";
    let buffer = File::create(&fin_name).unwrap();

    union(buffer, &mut contents, files.flag);

    print!("Files ");
    for i in 0..files.file_path.len()
    {
        print!("\"{}\" ", files.file_path[i]);
    }
    println!("union in \"{}\"", fin_name);

    remove_postf::remove_postfix(fin_name);
}

fn union(mut buffer: File, contents: &mut Vec<String>, flag:bool){
    let mut splits_vec: Vec<Vec<String>> = Vec::new();

    for i in 0..contents.len()-1{
        for j in i+1..contents.len(){
            if contents[i].lines().nth(0).unwrap().split(',').count() < contents[j].lines().nth(0).unwrap().split(',').count(){
                contents.swap(j, i);
            } 
        }
    }

    for i in 0..contents.len()
    {
        let mut spl: Vec<String> = Vec::new();
        for line in contents[i].lines()
        {
            spl.push(line.to_string());
        }
        splits_vec.push(spl.clone());
    }


    let mut vec_of_lines = non_same_columns(&splits_vec);

    buffer.write_all((vec_of_lines[0][0].clone()+"\n").as_bytes()).unwrap();

    for i in 0..vec_of_lines.len(){
        vec_of_lines[i].remove(0);
    }

    for i in 0..vec_of_lines.len()-1{

        // looking for lines that are in at least one of the files
        for line0 in vec_of_lines[i].clone().iter() {
            //divide the line into words, removing " and ,
            let words: Vec<&str> = line0.split("\",").collect();
            let mut words_vec: Vec<String> = words.iter().map(|x| x.trim_matches('"').to_string()).collect();
            words_vec.remove(words_vec.len()-1); //remove last empty word
            let mut same = false;
            for i1 in i+1..contents.len()
            {
                if contents[i1].contains(&words_vec[0])
                {
                    same = true
                }
            }
            if same
            {
                search_same(&mut vec_of_lines, i, &words_vec, &mut buffer, flag);
                continue;
            }
            buffer.write_all((line0.to_string()+"\n").as_bytes()).unwrap();
        }

    }


    for line1 in vec_of_lines[vec_of_lines.len()-1].iter() {
        buffer.write_all((line1.to_string()+"\n").as_bytes()).unwrap();
    }

}

#[allow(unused_assignments)]
fn search_same(vec_of_lines: &mut Vec<Vec<String>>, num:usize, need: &Vec<String>, buffer: &mut File, flag:bool)
{
    'top_for: for i in num+1..vec_of_lines.len()
    {
        let mut fin_str:String=String::new();
        let mut del_num = 0; // index of the line to be deleted in the other files
        
        // looking for the need line and index of it in vec_of_lines
        let need_in_vec = match vec_of_lines[i].iter().find(|x| {del_num+=1; x.contains(&need[0][..])})
        {
            Some(x) => x,
            None => continue,
        };
        del_num-=1;


        let words: Vec<&str> = need_in_vec.split("\",").collect();
        let mut words_vec: Vec<String> = words.iter().map(|x| x.trim_matches('"').to_string()).collect();
        words_vec.remove(words_vec.len()-1);

        // collect a string of two files and if there are excellent values and there is
        // no flag, then we return " " and write both lines from two files to union.csv
        fin_str = make_fin_str(&words_vec, need, flag);
        if fin_str==" " {
            let need_in_vec = need_in_vec.to_string() + "\n";
            buffer.write_all(need_in_vec.as_bytes()).unwrap();
            let mut need_str:String = String::new();
            need.iter().for_each(|x| need_str = need_str.clone() + "\""+ x + "\",");
            need_str = need_str + "\n";                                    
            buffer.write_all(need_str.as_bytes()).unwrap();
            vec_of_lines[i].remove(del_num);
            continue 'top_for;
        }

        fin_str = fin_str +  "\n";
        // delete line in the file that we used
        vec_of_lines[i].remove(del_num);


        //we look through the other files to see if the desired line is there and if there is, we take the final line
        for n in i+1..vec_of_lines.len()
        {
            let mut del_num = 0;
            let mut new_fin_str = String::new();
            
            let need_in_vec = match vec_of_lines[n].iter().find(|x| {del_num+=1; x.contains(&need[0][..])})
            {
                Some(x) => x,
                None => continue,
            };
            let splits: Vec<&str> = need_in_vec.split("\",").collect();
            let mut words_vec: Vec<String> = splits.iter().map(|x| x.trim_matches('"').to_string()).collect();
            words_vec.remove(words_vec.len()-1);

            let need: Vec<&str> = fin_str.split("\",").collect();
            let mut need_vec: Vec<String> = need.iter().map(|x| x.trim_matches('"').to_string()).collect();
            need_vec.remove(need_vec.len()-1);
            
            new_fin_str = make_fin_str(&words_vec, &need_vec, flag);
            if new_fin_str == " "
            {
                let need_in_vec = need_in_vec.to_string() + "\n";
                buffer.write_all(need_in_vec.as_bytes()).unwrap();
                let mut need_str:String = String::new();
                need.iter().for_each(|x| need_str = need_str.clone() + "\""+ x + "\",");
                need_str = need_str + "\n";                    
                buffer.write_all(need_str.as_bytes()).unwrap();
                vec_of_lines[i].remove(del_num);
                continue 'top_for;
            }
            fin_str = new_fin_str;
            vec_of_lines[n].remove(del_num);
        }

        buffer.write_all(fin_str.as_bytes()).unwrap(); //write final line in union.csv
    }
}

fn make_fin_str(split_vec: &Vec<String>, need: &Vec<String>, flag:bool) -> String
{
    let mut fin_str:String=String::new();
    for n in 0..split_vec.len()
    {
        if n==1 //in the second column quantiti so that we add them up
        {
            let number = need[n].parse::<i32>().unwrap()+split_vec[n].parse::<i32>().unwrap();
            fin_str = fin_str + "\"" + &number.to_string() + "\",";
            continue;
        }
        if need[n]==split_vec[n]
        {
            fin_str = fin_str + "\"" + &split_vec[n] + "\",";
        }else
        {
            if flag
            {
                fin_str=fin_str+"\"Forced merge\",";
            }else
            {
                return " ".to_string();
            }
        }
    }
    return fin_str; // return fin_str
}

fn non_same_columns(splits_vec: &Vec<Vec<String>>) -> Vec<Vec<String>>
{
    let mut num_ignr: Vec<Vec<i32>>= Vec::new(); // vector columns that we want to remove from the table

    let mut temp:Vec<i32>;
    //consider which columns are not in other tables in each file except the last one
    for i in 0..splits_vec.len()
    {
        let mut num_del = 0;
        temp=Vec::new();
        for j in i+1..splits_vec.len(){
            for words0 in splits_vec[i][0].split(",")
            {
                if !splits_vec[j][0].contains(words0)
                {
                    temp.push(num_del);
                }
                num_del+=1;
            }
            num_del=0;
        }
        num_ignr.push(temp.clone());
    }

    //delete all those columns from the tables, while creating temporary files for each of the tables
    return del_non_same_columns(splits_vec, &num_ignr);
}

fn del_non_same_columns(splits_vec: &Vec<Vec<String>>, num_ignr: &Vec<Vec<i32>>) -> Vec<Vec<String>>
{
    let mut new_csv_vec: Vec<Vec<String>> = Vec::new();
    for index_ignr in 0..num_ignr.len(){

        let mut new_file: Vec<String>=Vec::new();
        if num_ignr[index_ignr].len()!=0{

            for line in splits_vec[index_ignr].iter() {
            
                let splits: Vec<&str> = line.split("\",").collect();
                let mut num_of_iteration=-1;
                //record only those columns that we should not delete
                let mut split_vec: Vec<String> = splits.iter().filter(|_| {
                                                                num_of_iteration+=1; !num_ignr[index_ignr].contains(&num_of_iteration)
                                                            })
                                                            .map(|x| x.trim_matches('"').to_string())
                                                            .collect();                                    
                split_vec.remove(split_vec.len()-1);
                let mut slc = String::new();
                // well, we write them to a file   
                for i in 0..(split_vec.len()-1)
                {
                    slc = slc + "\""+&split_vec[i]+"\",";
                }
        
                slc = slc + "\"" + &split_vec[split_vec.len()-1]+"\",";
                new_file.push(slc);
            }

        }else{
            new_csv_vec.push(splits_vec[index_ignr].clone());
            continue;
        }
        new_csv_vec.push(new_file)

    }
    return new_csv_vec;
}