use std::fmt::Debug;
use std::io;
use std::ops::Deref;

use dyn_partial_eq::DynPartialEq;
use itertools::iproduct;
use tracing::{info, warn};

#[tracing::instrument]
pub fn task_2022_task3() {
    println!("std in!");
    let mut rows_and_columns = String::new();
    io::stdin().read_line(&mut rows_and_columns).expect("panic!");

    let mut split_rows_and_columns = rows_and_columns.split_whitespace();

    let rows: i32 = split_rows_and_columns.next().expect("panic!").parse().unwrap();
    let columns: i32 = split_rows_and_columns.next().expect("panic!").parse().unwrap();
    println!("rows: {}, columns: {}", rows, columns);

    let mut original_pattern: Vec<Vec<Color>> = Vec::new();
    for _ in 0..rows {
        let mut read_color_row = String::new();
        io::stdin().read_line(&mut read_color_row).expect("panic!");
        let color_row = read_color_row.chars().into_iter()
            .filter(|c| !c.is_control())
            .map(|c| Color::from_string(&c.to_string()))
            .collect();
        original_pattern.push(color_row);
    }

    let mut to_be_pattern: Vec<Vec<Color>> = Vec::new();
    for _ in 0..rows {
        let mut read_color_row = String::new();
        io::stdin().read_line(&mut read_color_row).expect("panic!");
        let color_row = read_color_row.chars().into_iter()
            .filter(|c| !c.is_control())
            .map(|c| Color::from_string(&c.to_string()))
            .collect();
        to_be_pattern.push(color_row);
    }
    println!("read");
    println!("{:?}", original_pattern);
    println!("{:?}", to_be_pattern);
    // return count(number, max_weight, items);
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Color {
    G,
    R,
    B,
}

impl Color {
    fn from_string(from_string: &str) -> Color {
        match from_string {
            "g" => Color::G,
            "r" => Color::R,
            "b" => Color::B,
            _ => panic!("invalid color"),
        }
    }
}

// type Pattern  = Vec<Vec<Color>>;

#[derive(DynPartialEq, PartialEq, Eq, Debug, Clone)]
struct Pattern {
    tiles: Vec<Vec<Color>>,
}

impl Pattern {
    fn horizontal_size(&self) -> usize {
        self.tiles.get(0).map_or(0, |v| v.len())
    }

    fn vertical_size(&self) -> usize {
        self.tiles.len()
    }

    fn get(&self, vertical_index: usize, horizontal_index: usize) -> Color {
        if vertical_index >= self.vertical_size() || horizontal_index >= self.horizontal_size() {
            panic!("invalid index")
        }
        self.tiles.get(vertical_index).unwrap().get(horizontal_index).unwrap().clone()
    }
}

// #[dyn_partial_eq]
trait TreeNode: PartialEq + Eq + 'static + Clone + Debug {
    fn get_children(&self) -> Vec<Box<Self>>;


    #[tracing::instrument]
    fn breadth_first_search_to(self, to_be: Box<Self>) -> i32 {
        // where NODE: TreeNode + PartialEq + Eq + 'static,
        //       Box<dyn TreeNode>: PartialEq<Box<NODE>> {
        let mut queue: Vec<Box<Self>> = Vec::new();
        let mut already_appeared: Vec<Box<Self>> = Vec::new();
        queue.push(Box::new(self.clone()));
        let mut depth = 0;
        while !queue.is_empty() {
            let mut next_queue: Vec<Box<Self>> = Vec::new();
            for node in queue {
                if node == to_be {
                    return depth;
                }

                if already_appeared.contains(&node) {
                    continue;
                } else {
                    next_queue.append(&mut node.get_children());
                    already_appeared.push(node)
                }
            }
            queue = next_queue;
            depth += 1;
        }
        return -1;
    }
}

type Range = (usize, usize);

fn create_ranges(start: usize, end_excluding: usize) -> Vec<Range> {
    println!("create ranges");
    let mut ranges: Vec<Range> = Vec::new();
    for i in start..end_excluding {
        for j in i..end_excluding {
            println!("create: {}: {}", i, j);
            ranges.push((i, j));
        }
    }
    ranges
}

impl TreeNode for Pattern {
    #[tracing::instrument]
    fn get_children(&self) -> Vec<Box<Self>> {
        let mut children: Vec<Box<Pattern>> = Vec::new();

        let vertical_ranges = create_ranges(0, self.vertical_size());
        let horizontal_ranges = create_ranges(0, self.horizontal_size());

        for (vertical, horizontal) in iproduct!(vertical_ranges, horizontal_ranges) {
            println!("vertical");
            println!("{}: {}", vertical.0, vertical.1);
            println!("horizontal");
            println!("{}: {}", horizontal.0, horizontal.1);
            // horizontally reversed pattern
            children.push(Box::new(line_symmetric_reverse(self, |row_index, column_index| {
                let new_row_index = row_index;
                let new_column_index = if column_index < horizontal.0 || column_index > horizontal.1 {
                    column_index
                } else {
                    horizontal.1 - (column_index - horizontal.0)
                };
                return (new_row_index, new_column_index);
            })));
            // vertically reversed pattern
            children.push(Box::new(line_symmetric_reverse(self, |row_index, column_index| {
                let new_row_index = if row_index < vertical.0 || row_index > vertical.1 {
                    row_index
                } else {
                    vertical.1 - (row_index - vertical.0)
                };
                let new_column_index = column_index;
                return (new_row_index, new_column_index);
            })));

            // square range case
            if vertical.1 - vertical.0 == horizontal.1 - horizontal.0 {
                children.push(Box::new(line_symmetric_reverse(self, |row_index, column_index| {
                    if row_index < vertical.0 || row_index > vertical.1 || column_index < horizontal.0 || column_index > horizontal.1 {
                        println!("bbbbbbbbbbbbbbbbbbbbbbbbb");
                        println!("{}: {}", row_index, column_index);
                        return (row_index, column_index)
                    } else {
                        let new_row_index = vertical.0 + (column_index - horizontal.0);
                        let new_column_index = horizontal.0 + (row_index - vertical.0);
                        println!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
                        println!("{}: {}", row_index, column_index);
                        println!("{}: {}", new_row_index, new_column_index);

                        return (new_row_index, new_column_index);
                    }
                })));

                children.push(Box::new(line_symmetric_reverse(self, |row_index, column_index| {
                    if row_index < vertical.0 || row_index > vertical.1 || column_index < horizontal.0 || column_index > horizontal.1 {
                        println!("bbbbbbbbbbbbbbbbbbbbbbbbb");
                        println!("{}: {}", row_index, column_index);
                        return (row_index, column_index)
                    } else {
                        let new_row_index = vertical.1 - (column_index - horizontal.0);
                        let new_column_index = horizontal.1 - (row_index - vertical.0);
                        return (new_row_index, new_column_index);
                    }
                })));
            }
        }
        for child in children.clone() {
            println!("child: {:?}", child)
        }
        return children;
    }
}

fn line_symmetric_reverse<F>(pattern: &Pattern, row_column_index_change_rule: F) -> Pattern
    where F: Fn(usize, usize) -> (usize, usize)  {
    let mut new_pattern = Vec::new();
    for row_index in 0..pattern.vertical_size() {
        let mut new_row: Vec<Color> = Vec::new();
        for column_index in 0..pattern.horizontal_size() {
            let (refer_row_index, refer_column_index) = row_column_index_change_rule(row_index, column_index);
            new_row.push(pattern.get(refer_row_index, refer_column_index))
        }
        new_pattern.push(new_row);
    }
    return Pattern { tiles: new_pattern };
}


#[cfg(test)]
mod tests {
    use crate::task2022::codingtasks::task3::{Color, Pattern, TreeNode};

    // #[test]
    // fn test_0() {
    //     create_indexed_bit_set(4).iter().for_each(|composition| {
    //         println!("{:?}", composition);
    //     });
    // }
    //
    #[test]
    fn test_1() {
        let mut original: Vec<Vec<Color>> = Vec::new();
        original.push(vec![Color::G, Color::G, Color::B]);
        original.push(vec![Color::R, Color::R, Color::G]);

        let mut tobe: Vec<Vec<Color>> = Vec::new();
        tobe.push(vec![Color::R, Color::B, Color::R]);
        tobe.push(vec![Color::G, Color::G, Color::G]);

        let original_pattern = Pattern { tiles: original };

        let times = original_pattern.breadth_first_search_to(Box::new(Pattern { tiles: tobe }));
        assert_eq!(times, 2)

    }
}
