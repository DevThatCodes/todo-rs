use core::str;
use std::{env::{self}, fs::{self, File}, process::exit};



fn main() {
    // todo-rs -req[FILENAME (ex. todolist.txt)] -opt[TODO THINGS (ex. code-something,code-something-else)]
    //
    // -req = required argument
    // -opt = optional argument

    // DONE: add error handling

    let args : Vec<String> = env::args().collect();
    // DONE: added error handling for if there is the FILENAME argument exists
    let todo_file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        println!("[ERROR] Enter the filename of the todolist (ex. todo-rs important-todos.txt)");
        exit(1);
    };
    // DONE: added error handling for if the file exists or #![no_std]
    if args.contains(&"--new".to_string()) {
        let _ = File::create(todo_file_path.clone());
    }
    let todo_file_content = match fs::read_to_string(&todo_file_path) {
        Ok(file_content) => {file_content}
        Err(..) => {println!("[ERROR] Enter a valid file, maybe you meant to create a new todolist with the --new flag?"); exit(1);}
    };
    
    let mut todo_file : Vec<&str> = Vec::new();
    todo_file_content.lines().for_each(|line| todo_file.push(line));

    let mut todo_things : Vec<&str> = Vec::new();
    // DONE: check if TODO THINGS arg exists. If so, split it into a vec by commas 
    if args.len() > 2 && args[2]!="--not-fancy" && args[2]!="--new" {
        args[2].split("~").for_each(|string| todo_things.push(string));
    }

    // DONE: for each thing in TODO THINGS check if its already in FILENAME. If so, remove it from
    // the file. Otherwise, add it to the file

    for thing in todo_things {
        if todo_file.contains(&thing) {
            todo_file.retain(|&line| line != thing);
        } else {
            todo_file.push(thing);
        }
    }

    fs::write(todo_file_path.clone(), todo_file.join("\n")).unwrap();
    if !args.contains(&"--not-fancy".to_string()) {
        println!("{}",todo_file_path.clone());
        for ele in todo_file {
            println!(" - {ele}");
        }
    } else {
        println!("{}", fs::read_to_string(todo_file_path).expect("File Couldn't open!"))
    }
}
