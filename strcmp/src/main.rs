use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Enter two strings");
    
        println!("Please input string 1");
        
        let mut str1 = String::new();
        let mut str2 = String::new();
        
        io::stdin().read_line(&mut str1)
        .ok()
        .expect("Failed to read line");
        
        println!("Please input string 2");
        io::stdin().read_line(&mut str2)
        .ok()
        .expect("Failed to read line");
        
        
        match str1.cmp(&str2){
        Ordering::Less      => println!("str1 is smaller than str2"),
        Ordering::Greater   => println!("str1 is bigger than str2"),
        Ordering::Equal     => println!("str1 is equal to str2"),
        }
}