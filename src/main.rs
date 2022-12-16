fn main() {
     
    // This is the first program in the course
    // This is the second line of comment

    /* This is a multiline comment
    like with C++ */

    print!("hello from Rust program!"); //like the c++ printf() function

    // learning some basic output commands
    println!("The value of the constant is {}",10);

    //learning to print strings
    print!("This is a print command");
    print!("This is going to be printed on the same line");
    println!("");

    // Learning to write on multiple lines using the print command
    println!("This is going to be 
    printed on multiple
    lines");

    // Learning the use of escape sequences
    println!("\n\n This is going to be printed after two lines");   // \n is a newline character like in c++
    
    // Learning the use of double escape characters
    println!("\\n\n This is printed '\\n' by escaping the character now");

    // Learning some uses of backslashes
    println!("This will print single quote \' and this will print double quote \"");
    println!("This is going to print one backslash \\ ");

}
