use std::io;

pub fn task_2022_task1() {
    println!("std in!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("panic!");
    println!("read");

    let mut split = input.split_whitespace();

    let n: i32 = split.next().expect("panic!2").parse().unwrap();
    let mut x: i32 = split.next().expect("panic!3").parse().unwrap();
    println!("n: {}", n);
    println!("x: {}", x);

    let mut count = 0;

    while x > 0 {
        let to_subtract = (0..n).map(|i| 2i32.pow(i as u32))
            .filter(|&i_powered_2| i_powered_2 <= x)
            .max().unwrap();
        x -= to_subtract;
        count += 1;
    }
    println!("{}", count);
}
