mod lib;

use std::{fs};
use lib::*;

fn main(){
    let content = fs::read_to_string("ama.file").expect("Corrupted file");// reads the file
    let c: Vec<&str> = content.as_str().split("\r\n").collect();//splits them by newline and puts them in a vector
    let number_of_cases = c[0].parse::<usize>().unwrap();// takes the first number i.e the number of cases
    let new = &c[2..]; // slice the numbers needed for the work
    different_cases(number_of_cases, new);// this sends the function the number of cases and the 
    //set of numbers
}