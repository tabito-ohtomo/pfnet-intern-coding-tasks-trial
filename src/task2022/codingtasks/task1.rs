use std::io;
use crate::util::break_down_to_2_powered_numbers;

pub fn task_2022_task1() {
    println!("std in!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("panic!");
    println!("read");

    let mut split = input.split_whitespace();

    let n: i32 = split.next().expect("panic!2").parse().unwrap();
    let x: i32 = split.next().expect("panic!3").parse().unwrap();
    println!("n: {}", n);
    println!("x: {}", x);

    let mut count = 0;

    count += x / 2i32.pow((n-1) as u32);
    count += break_down_to_2_powered_numbers(x % 2i32.pow((n-1) as u32), n).iter().filter(|&(_, &consist_or_not)| consist_or_not).count();

    println!("{}", count);
}
