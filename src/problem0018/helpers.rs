use std::fs::File;
use std::io::{BufReader, BufRead};
use std::cmp;

pub fn get_tree(filename:&str) -> Vec<Vec<u32>> {
    let file = File::open(filename).unwrap();

    let filebuf = BufReader::new(&file);

    let mut tree:Vec<Vec<u32>> = vec![];
    for line in filebuf.lines() {
        let l = line.unwrap();

        let values = l.split(" ").map(|x| x.parse::<u32>().unwrap()).collect();

        tree.push(values);
    }

    tree
}

pub fn max_tree_path(tree:&Vec<Vec<u32>>) -> u32 {
    // Start at the bottom, and work up, storing the largest path for each previous row

    let mut working_tree:Vec<Vec<u32>> = vec![];

    for (row_index, row) in tree.iter().rev().enumerate() {
        println!("Row {}", row_index);

        println!("working_tree in {} long", working_tree.len());
        if row_index == 0 {
            // Push the row as is if it's the first one
            working_tree.push(row.clone());
        } else {
            let prev_row = working_tree.last().unwrap().clone();

            let mut working_row:Vec<u32> = vec![];
            for (column_index, value) in row.iter().enumerate() {
                println!("{}: {} or {}", value, prev_row[column_index], prev_row[column_index + 1]);
                working_row.push(cmp::max(value + prev_row[column_index], value + prev_row[column_index + 1]))
            }

            working_tree.push(working_row);
        }
    }

    *working_tree.last().unwrap().last().unwrap()
}
