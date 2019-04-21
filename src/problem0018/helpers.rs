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
    let mut working_tree:Vec<Vec<u32>> = tree.clone();

    for row_index in (1..working_tree.len()).rev() {
        for column_index in (1..working_tree[row_index].len()).rev() {
            working_tree[row_index - 1][column_index - 1] += cmp::max(working_tree[row_index][column_index - 1], working_tree[row_index][column_index])
        }
    }

    tree[0][0]
}
