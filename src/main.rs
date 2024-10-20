use core::str;
use std::{env::{self}, fs};

fn main() {
    // todo-rs -req[FILENAME (ex. todolist.txt)] -opt[TODO THINGS (ex. code-something,code-something-else)]
    //
    // -req = required argument
    // -opt = optional argument

    let args : Vec<String> = env::args().collect();
    let todo_file_path = args[1].clone();
    let todo_file_content = fs::read_to_string(&todo_file_path).expect("Couldn't open file!");
    
    let mut todo_file : Vec<&str> = Vec::new();
    todo_file_content.lines().for_each(|line| todo_file.push(line));

    let mut todo_things : Vec<&str> = Vec::new();
    // DONE: check if TODO THINGS arg exists. If so, split it into a vec by commas 
    if args.len() > 2 && args[2]!="--fancy" {
        args[2].split(",").for_each(|string| todo_things.push(string));
    }

    // PARTIAL: for each thing in TODO THINGS check if its already in FILENAME. If so, remove it from
    // the file. Otherwise, add it to the file
    //
    // BUG: if a thing to be removed appears first, and once removed the file is empty, anything
    // after that is ignored (ex. 
    //
    //  todo.txt:
    //  |thing
    //
    //  todo-rs todo.txt thing,other-thing => makes todo.txt empty instead of having other-thing in
    //  it
    // )

    for thing in todo_things {
        if todo_file.contains(&thing) {
            todo_file.retain(|&line| line != thing);
        } else {
            todo_file.push(thing);
        }
    }

    fs::write(todo_file_path.clone(), todo_file.join("\n")).unwrap();
    if args.contains(&"--fancy".to_string()) {
        println!("{}",todo_file_path.clone());
        for ele in todo_file {
            println!(" - {ele}");
        }
    } else {
        println!("{}", fs::read_to_string(todo_file_path).expect("File Couldn't open!"))
    }
}
