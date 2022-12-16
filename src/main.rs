fn main() {
     
    // This is the first program in the course
    // This is the second line of comment

    /* This is a multiline comment
    like with C++ */

    print!("Hello from the Rust program!"); //like the c++ printf() function
    println!("");

    // learning some basic output commands
    println!("The value of the constant is {}",10);

    //learning to print strings
    print!("This is a print command. ");
    print!("This is going to be printed on the same line");
    println!("");

    // Learning to write on multiple lines using the print command
    println!("This is going to be 
    printed on multiple
    lines");

    // Learning the use of escape sequences
    println!("1\n2\n This is going to be printed after two lines");   // \n is a newline character like in c++
    
    // Learning the use of double escape characters
    println!("\\n\nThis is printed '\\n' by escaping the character now");

    // Learning some uses of backslashes
    println!("This will print single quote \' and this will print double quote \"");
    println!("This is going to print one backslash \\");

    // Learning the user of \r to overwrite text
    print!("This is some text which will be overwritten \rThis text will only appear on the output terminal");

    // Learning the positional argument
    println!("\nI have been doing {2} for {1} years and I {0} it", "like", 20, "programming");

    // Learning named arguments
    println!("{language} is a system programming language which is cool to {activity} in", language = "Rust", activity = "code");

    //Learning how to print basic math
    println!("The summation of 25 + 10 = {}", 25+10);

}
