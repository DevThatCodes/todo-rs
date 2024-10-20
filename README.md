# What todo-rs is
todo-rs is a small CLI todo list manager that uses files as todo lists.
# How to use
run the command: `todo-rs -req[FILENAME] -opt[THINGS TO DO]`.
arguments marked by -req are required. Similarly, arguments marked by -opt are optional.
### To add things to the list
write it with no spaces and each thing to add to the list seperated by commas.<br>
ex: `todo-rs todo.txt code-something,eat-a-snack`
### To remove things from the list
just write it in the THINGS TO DO argument and it will be removed from the list.<br>
ex `todo-rs todo.txt code-something`
### To check the list
run the command `todo-rs todo.txt`
# If you want to have the command accessible
put the directory that you have todo-rs in your system's PATH variable
