// Rust standard library (std) comes with
//  (env) module which allows access to cmd arguments
//  these are (args) function and (Args) struct
//  args function returns instance of the Args struct
use std::env::{args, Args};

fn main() {
  // declare an 'args' variable
  //  optionally, we explicitly type the args variable
  //  with Args
  let args1: Args = args();

  // the '{}' syntax can be used to print the contents of 'args'
  //   to format args into a string the ':?' is needed because 
  //   args() doesn't implement the std::fmt::Display trait
  println!("{:?}", args1);
  println!("{:#?}", args1); // ':#?' is used to pretty-print

  // Use args to perform add, subtract, multiply or divide on two numbers
  //  printing the math expression with the result
  
  // declare another 'args' variable as mutable with 'mut'
  let mut args2: Args = args();
  //  accessing the first user argument requires skipping the
  //  first run directory arg
  let first = args2.nth(1).unwrap();
  //  nth uses an internal iterator, so the next index after
  //  the last unwrapped item will be the 0-th element
  //  with the current iterator position
  //  - we'd also like 'operator' to be type char
  //    so we get an iterator on the unwrapped string with .chars()
  //    then we use .next() to iterate on the first element in the string
  //    and we .unwrap() that element, returning the char
  let operator = args2.nth(0).unwrap().chars().next().unwrap();
  let second = args2.nth(0).unwrap();

  // the parse method parses the string into another type
  //  we can use the variable type annotation like this
  let first_number: f32 = first.parse().unwrap();
  //  or we can use turbofish syntax on the parse method
  let second_number = second.parse::<f32>().unwrap();

  let result = operate(operator, first_number, second_number);
  
  println!("{}", output(first_number, operator, second_number, result));
}

// implicit return, without return keyword or semi-colon
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  // pattern matching on operator
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '*' | 'x' | 'X' => first_number * second_number, // bitwise OR extends match case
    '/' => first_number / second_number,
    _ => panic!("Invalid operator used.") //base case
  }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!("{} {} {} = {}", first_number, operator, second_number, result)
}
