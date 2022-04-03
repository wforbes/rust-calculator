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

}