# lib_andre

##Current functions:

###lib_andre::io

*Prints the argument to the screen like a prompt and waits

for the users input, then saves it to a string and returns it.*

pub fn prompt(ps: &str) -> Result<String, io::Error> 

Example:
`let response = try!(prompt("Name?: "));`


*Takes a file name as a string and prints the contents of the file

if it exists.*

pub fn print_file(file_name: &str) -> Result<String, Box<Error>> 

Example:
`print_file("/home/andre/sample.txt");`
