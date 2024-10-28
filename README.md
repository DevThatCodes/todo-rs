# What todo-rs is
todo-rs is a small CLI todo list manager that uses files as todo lists. <br>
**NOTE: there is currently only a linux and a windows version, And i am aware of the issue where microsoft thinks the windows version is a virus. I don't plan on adding a mac os version, if you want to use this on mac os, you will have to build it from source**
# How to download
get the latest todo-rs version [here](https://github.com/DevThatCodes/todo-rs/releases/latest)
# How to use
run the command: `./todo-rs -req[FILENAME] -opt[THINGS TO DO] -opt[FLAGS]`.
arguments marked by -req are required. Similarly, arguments marked by -opt are optional.
#### Current Flags:
 - `--not-fancy`: makes the list look worse
 - `--new`: makes a new todolist
### Removed flags:
 - `--fancy`, reason why removed: now it is the default way of displaying the todolist
### To check the list
run the command `./todo-rs [FILENAME]` where `[FILENAME]` is equal to the path of the todo-list file.
![2024-10-21-16-52-34](https://github.com/user-attachments/assets/70c4a268-5027-4516-85a6-c81f2810b6f6)

### To add things to the list
write it with no spaces and each thing to add to the list seperated by hyphens.<br>
ex: `./todo-rs [FILENAME] [THING-TO-ADD]` where `[FILENAME]` is equal to the path of the todo-list file, and `[THING-TO-ADD]` is equal to the name of the thing you want to add to the list.<br> **NOTE: make sure the name of the thing to add doesn't have any spaces, otherwise it will not work. alternatively you can put the things in single quotes to use spaces**
![2024-10-21-17-02-55](https://github.com/user-attachments/assets/625315ce-2fae-43e5-afef-1f32c8ba91b7)

### To remove things from the list
just write it in the THINGS TO DO argument and it will be removed from the list.<br>
ex `./todo-rs [FILENAME] [THING-TO-REMOVE]` where `[FILENAME]` is equal to the path of the todo-list file, and `[THING-TO-REMOVE]` is equal to the name of the thing you want to remove from the list.
![2024-10-21-17-03-58](https://github.com/user-attachments/assets/42340b41-9b40-48c5-a78c-5f3a4b1abb24)

# If you want to have the command accessible from anywhere
put the directory that you have todo-rs in your system's PATH variable

# Troubleshooting
### Linux: If you get a permission denied error
run the command `chmod +x todo-rs` to give the executable permissions to the file
### Linux: If you get an error like this `bash: ./todo-rs: Is a directory`
you most likely cloned the git repository, download the todo-rs file from [here](https://github.com/DevThatCodes/todo-rs/releases/latest)
