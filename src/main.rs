#![allow(non_snake_case)]
use std::{fs, vec};
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
        let binding = "smart speaker v.1 bom.csv".to_string();
        let binding2 = "smart speaker v.2 bom.csv".to_string();
        let files=Files
        {
            file_path: vec![&binding, &binding2],
            flag: false,
        };
        run (files);
    }
}

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

    if args[args.len()-1]=="-f"
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
    
    union(&mut contents, files.flag);
    print!("Files ");
    for i in 0..files.file_path.len()
    {
        print!("\"{}\" ", files.file_path[i]);
    }
    println!("union in \"union.csv\"");

    remove_postf::remove_postfix(fs::read_to_string("union.csv").unwrap());
}

fn union(contents: &mut Vec<String>, flag:bool){
    let mut splits_vec: Vec<Vec<String>> = Vec::new();
    for i in 0..contents.len()
    {
        let mut spl: Vec<String> = Vec::new();
        for line in contents[i].lines()
        {
            spl.push(line.to_string());
        }
        splits_vec.push(spl.clone());
    }
    non_same_columns(&splits_vec);
    

    for num in 0..contents.len()
    {
        let path = "temp".to_string() + &num.to_string() +".csv";
        if Path::new(&path).exists()
        {
            contents[num] = fs::read_to_string(&path)
                            .expect("Should have been able to read the file");
        }
    }

    let mut buffer = File::create("union.csv").unwrap();
    buffer.write_all((contents[0].lines().nth(0).unwrap().to_string()+"\n").as_bytes()).unwrap();

    let mut vec_of_lines:Vec<Vec<&str>> = Vec::new();

    let mut len = 0;
    for cont in contents.iter()
    {
        vec_of_lines.push(cont.lines().into_iter().collect());
        vec_of_lines[len].remove(0);
        len+=1;
    }


    for line0 in vec_of_lines[0].clone().iter() {
        
        let splits: Vec<&str> = line0.split("\",").collect();
        let mut split_vec: Vec<String> = splits.iter().map(|x| x.trim_matches('"').to_string()).collect();
        split_vec.remove(split_vec.len()-1);
        let mut same = false;
        for i1 in 1..contents.len()
        {
            if contents[i1].contains(&split_vec[0])
            {
                same = true
            }
        }
        if same
        {
            search_same(&mut vec_of_lines, &split_vec, &mut buffer, flag);
            continue;
        }
        buffer.write_all((line0.to_string()+"\n").as_bytes()).unwrap();
    }


    for i in 1..vec_of_lines.len()
    {
        for line1 in vec_of_lines[i].iter() {
            buffer.write_all((line1.to_string()+"\n").as_bytes()).unwrap();
        }
    }

    for num in 0..vec_of_lines.len()
    {
        let path = "temp".to_string() + &num.to_string() +".csv";
        if Path::new(&path).exists()
        {
            fs::remove_file(&path).unwrap();
        }
    }

}

#[allow(unused_assignments)]
fn search_same(vec_of_lines: &mut Vec<Vec<&str>>, need: &Vec<String>, buffer: &mut File, flag:bool)
{
    'top_for: for i in 1..vec_of_lines.len()
    {
        let mut fin_str:String=String::new();
        let mut del_num = 0;
        let need_in_vec = match vec_of_lines[i].iter().find(|x| {del_num+=1; x.contains(&need[0][..])})
        {
            Some(x) => x,
            None => continue,
        };
        del_num-=1;
        let splits: Vec<&str> = need_in_vec.split("\",").collect();
        let mut split_vec: Vec<String> = splits.iter().map(|x| x.trim_matches('"').to_string()).collect();
        split_vec.remove(split_vec.len()-1);

        fin_str = make_fin_str(&split_vec, need, flag);
        if fin_str==" "{
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
        vec_of_lines[i].remove(del_num);

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
            let mut split_vec: Vec<String> = splits.iter().map(|x| x.trim_matches('"').to_string()).collect();
            split_vec.remove(split_vec.len()-1);

            let need: Vec<&str> = fin_str.split("\",").collect();
            let mut need_vec: Vec<String> = need.iter().map(|x| x.trim_matches('"').to_string()).collect();
            need_vec.remove(need_vec.len()-1);
            
            new_fin_str = make_fin_str(&split_vec, &need_vec, flag);
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
        buffer.write_all(fin_str.as_bytes()).unwrap();
    }
}

fn make_fin_str(split_vec: &Vec<String>, need: &Vec<String>, flag:bool) -> String
{
    let mut fin_str:String=String::new();
    for n in 0..split_vec.len()
    {
        if n==1
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
    return fin_str;
}

fn non_same_columns(splits_vec: &Vec<Vec<String>>)
{
    let mut num_ignr: Vec<Vec<i32>>= Vec::new();

    let mut temp:Vec<i32>;
    for i in 0..splits_vec.len()-1
    {
        let mut num_del = 0;
        temp=Vec::new();
        for words0 in splits_vec[i][0].split(",")
        {
            if !splits_vec[i+1][0].contains(words0)
            {
                temp.push(num_del);
            }
            num_del+=1;
        }
        num_ignr.push(temp.clone());
    }
    
    temp = Vec::new();
    let mut num_del = 0;
    for words0 in splits_vec[splits_vec.len()-1][0].split(",")
    {
        if !splits_vec[splits_vec.len()-2][0].contains(words0)
        {
            temp.push(num_del);
        }
        num_del+=1;
    }
    num_ignr.push(temp);

    let mut nums = 0;
    for ignr in num_ignr.clone()
    {
        if ignr.len()!=0
        {
            create_temp(splits_vec, &num_ignr, nums);
        }
        nums+=1;
    }
}

fn create_temp(splits_vec: &Vec<Vec<String>>, num_ignr: &Vec<Vec<i32>>, num: usize)
{
    let mut buffer = File::create("temp".to_string()+&num.to_string()+".csv").unwrap();
    for line in splits_vec[num].iter() {
    
        let splits: Vec<&str> = line.split("\",").collect();
        let mut num_of_iteration=-1;
        let mut split_vec: Vec<String> = splits.iter().filter(|_| {
                                                        num_of_iteration+=1; !num_ignr[num].contains(&num_of_iteration)
                                                    })
                                                    .map(|x| x.trim_matches('"').to_string())
                                                    .collect();
        split_vec.remove(split_vec.len()-1);                                            
        for i in 0..(split_vec.len()-1)
        {
            let slc = "\"".to_string()+&split_vec[i]+"\",";
            buffer.write_all(slc.as_bytes()).unwrap();
        }
        let slc = "\"".to_string() + &split_vec[split_vec.len()-1]+"\",\n";
        buffer.write_all(slc.as_bytes()).unwrap();
    }
}