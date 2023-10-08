use std::collections::HashMap;

// return : key: power of 2, value: whether to consist or not
pub fn break_down_to_2_powered_numbers(to_split: i32, number_of_2_powered_numbers: i32) -> HashMap<i32, bool> {
    let mut two_powered_numbers_to_consist_or_not: HashMap<i32, bool> = HashMap::new();
    // println!("{}", to_split);
    // println!("{}", number_of_2_powered_numbers);
    if 2i32.pow(number_of_2_powered_numbers as u32) <= to_split {
        panic!("to_split is too big for number_of_2_powered_numbers");
    }

    // initialize
    for i in 0..number_of_2_powered_numbers {
        two_powered_numbers_to_consist_or_not.insert(i, false);
    }

    let mut current_number = to_split.clone();

    while current_number > 0 {
        let to_subtract_power = two_powered_numbers_to_consist_or_not.keys()
            .filter(|&&two_powered| 2i32.pow(two_powered as u32) <= current_number)
            .max().unwrap();
        current_number -= 2i32.pow(*to_subtract_power as u32);
        two_powered_numbers_to_consist_or_not.insert(*to_subtract_power as i32, true);
    }
    two_powered_numbers_to_consist_or_not
}
