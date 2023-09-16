use std::collections::HashMap;
use std::io;
use std::iter::Map;
use crate::util::break_down_to_2_powered_numbers;

pub fn task_2022_task2() {
    println!("std in!");
    let mut number_and_max_weight = String::new();
    io::stdin().read_line(&mut number_and_max_weight).expect("panic!");

    let mut split_number_and_max_weight = number_and_max_weight.split_whitespace();

    let number: i32 = split_number_and_max_weight.next().expect("panic!").parse().unwrap();
    let max_weight: i32 = split_number_and_max_weight.next().expect("panic!").parse().unwrap();
    println!("number: {}, max_weight: {}", number, max_weight);

    let mut items: HashMap<i32, Item> = HashMap::new();

    let mut index = 0i32;
    loop {
        index += 1;
        let mut weight_value_evaluation = String::new();
        io::stdin().read_line(&mut weight_value_evaluation).expect("panic!");

        if weight_value_evaluation.trim().is_empty() {
            break;
        }
        println!("{}", weight_value_evaluation.trim().len());

        let mut split_weight_value_evaluation = weight_value_evaluation.split_whitespace();

        let weight: i32 = split_weight_value_evaluation.next().expect("panic!").parse().unwrap();
        let value: i32 = split_weight_value_evaluation.next().expect("panic!").parse().unwrap();
        let evaluation: i32 = split_weight_value_evaluation.next().expect("panic!").parse().unwrap();
        println!("weight: {}, value: {}, evaluation: {}", weight, value, evaluation);

        items.insert(index, Item {
            weight,
            value,
            evaluation,
        });
    }
    println!("read");
    return count(number, max_weight, items);
}

pub fn count(number_of_items: i32, max_weight: i32, items: HashMap<i32, Item>) {
    let compositions: Vec<HashMap<i32, bool>> = create_indexed_bit_set(number_of_items);

    let mut composition_to_value_and_evaluation: HashMap<i32, (i32, i32)> = HashMap::new();
    for (composition_index, &ref composition) in compositions.iter().enumerate() {
        let sum_of_weight: i32 = composition.iter()
            .filter(|(&_, &consist_or_not)| consist_or_not)
            .map(|(&item_index_minus_one, &consist_or_not)| {
                items.get(&(item_index_minus_one + 1)).unwrap().weight
            }).sum();
        if sum_of_weight > max_weight {
            continue;
        }

        let sum_of_value = composition.iter()
            .filter(|(&_, &consist_or_not)| consist_or_not)
            .map(|(&item_index_minus_one, &consist_or_not)| {
                // println!("item_index: {}", item_index);
                items.get(&(item_index_minus_one + 1)).unwrap().value
            }).sum();

        let sum_of_evaluation = composition.iter()
            .filter(|(&_, &consist_or_not)| consist_or_not)
            .map(|(&item_index_minus_one, &consist_or_not)| {
                items.get(&(item_index_minus_one + 1)).unwrap().evaluation
            }).sum();
        //
        // println!("composition_index: {}, sum_of_weight: {}, sum_of_value: {}, sum_of_evaluation: {}", composition_index, sum_of_weight, sum_of_value, sum_of_evaluation);
        // composition.iter().for_each(|(&item_index_minus_one, &consist_or_not)| {
        //     println!("item_index: {}, consist_or_not: {}", item_index_minus_one + 1, consist_or_not);
        // });
        //
        composition_to_value_and_evaluation.insert(composition_index as i32, (sum_of_value, sum_of_evaluation));
    }

    let count = composition_to_value_and_evaluation.iter().filter(|(&composition_index_of_x, &(value_of_x, evaluation_of_x))| {
        composition_to_value_and_evaluation.iter()
            .filter(|(&composition_index_of_y, &_)| {
                !(composition_index_of_x == composition_index_of_y)
            })
            .all(|(&composition_index_of_y, &(value_of_y, evaluation_of_y))| {
                (value_of_x > value_of_y || evaluation_of_x > evaluation_of_y) ||
                    (value_of_x == value_of_y && evaluation_of_y == evaluation_of_y)
            })
    }).count();

    println!("{}", count);
}


pub fn create_indexed_bit_set(size: i32) -> Vec<HashMap<i32, bool>> {
    let mut bit_set_vec: Vec<HashMap<i32, bool>> = Vec::new();
    for i in 1..2i32.pow(size as u32) {
        bit_set_vec.push(break_down_to_2_powered_numbers(i, size));
    }
    bit_set_vec
}

pub struct Item {
    weight: i32,
    value: i32,
    evaluation: i32,
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::task2022::codingtasks::task2::{count, create_indexed_bit_set, Item};

    #[test]
    fn test_0() {
        create_indexed_bit_set(4).iter().for_each(|composition| {
            println!("{:?}", composition);
        });
    }

    #[test]
    fn test_1() {
        let mut input: HashMap<i32, Item> = HashMap::new();
        input.insert(1, Item {
            weight: 3,
            value: 1,
            evaluation: 4,
        });
        input.insert(2, Item {
            weight: 3,
            value: 4,
            evaluation: 1,
        });
        input.insert(3, Item {
            weight: 2,
            value: 2,
            evaluation: 2,
        });
        input.insert(4, Item {
            weight: 4,
            value: 3,
            evaluation: 3,
        });

        count(4, 7, input);
    }

    #[test]
    fn test_2() {
        let mut input: HashMap<i32, Item> = HashMap::new();
        input.insert(1, Item {
            weight: 1,
            value: 1,
            evaluation: 5,
        });
        input.insert(2, Item {
            weight: 1,
            value: 2,
            evaluation: 4,
        });
        input.insert(3, Item {
            weight: 1,
            value: 3,
            evaluation: 3,
        });
        input.insert(4, Item {
            weight: 1,
            value: 4,
            evaluation: 2,
        });
        input.insert(5, Item {
            weight: 1,
            value: 5,
            evaluation: 1,
        });

        count(5, 2, input);
    }
}