//
// main_test.rs
// Copyright (C) 2017 huang <huang@huang-Precision-WorkStation-T3500>
// Distributed under terms of the MIT license.
//

#![feature(plugin)]
#![plugin(overflower)]


#![overflow(wrap)] //errors here!:w

extern crate overflower_support;

struct AA {
    x: i32,
}

macro_rules! hello_AA {
    () => ( AA {x: 3});
}  

//#[overflow(wrap)]
fn main() {
    let x = hello_AA!();
    println!("x.x = {}",x.x);
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
