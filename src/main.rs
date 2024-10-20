use std::{env, fs};

fn main() {
    // todo-rs -req[FILENAME (ex. todolist.txt)] -opt[TODO THINGS (ex. code-something,code-something-else)]
    //
    // -req = required argument
    // -opt = optional argument

    let args : Vec<String> = env::args().collect();
    let todo_file_path = args[1].clone();
    let mut todo_file = fs::read_to_string(&todo_file_path).expect("Couldn't open file!");
    // DONE: if todo_file has nothing in it, make it so that it can be written to
    if todo_file.is_empty() {
        todo_file = "\n".to_string();
    }
    let mut todo_things : Vec<&str> = Vec::new();
    // DONE: check if TODO THINGS arg exists. If so, split it into a vec by commas 
    if args.len() > 2 {
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
        for line in todo_file.clone().lines() {
            if line == thing {
                todo_file = todo_file.replace(thing, "").trim().to_string();
                println!("--deleted {}!", thing);
                break;
            } else if !todo_file.contains(thing) {
                 if todo_file.is_empty() {
                    todo_file = thing.to_string();
                }
                todo_file = format!("{}\n{}", todo_file, thing).trim().to_string();
                println!("--added {}!", thing);
           }
        }
    }

    fs::write(todo_file_path, &todo_file).unwrap();
    todo_file.lines().for_each(|line| println!("|{line}"));
}
