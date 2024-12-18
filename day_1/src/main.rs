use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[macro_export]
macro_rules! add_to_binheap {
    ($a:expr, $b:expr) => {
        $a.push(Reverse($b.parse::<u32>().unwrap()))
    }
} 

fn main() {
    part_1("input.txt").unwrap();
    part_2("input.txt").unwrap();
}

fn part_1(file_name: &str) -> io::Result<()> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut data_1 = BinaryHeap::new();
    let mut data_2 = BinaryHeap::new();
    let mut result = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let values: Vec<_> = line.split_whitespace().collect();
        add_to_binheap!(data_1, values[0]); 
        add_to_binheap!(data_2, values[1]); 
    }
    for _ in 0..data_1.len() {
        result += (data_1.pop().unwrap().0).abs_diff(data_2.pop().unwrap().0);
    }
    println!("{}", result);
    Ok(())
}

fn part_2(file_name: &str) -> io::Result<()> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut data_1 = Vec::new();
    let mut data_2 = Vec::new();
    let mut result = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        let values: Vec<_> = line.split_whitespace().collect();
        data_1.push(values[0].parse::<u32>().unwrap()); 
        data_2.push(values[1].parse::<u32>().unwrap());
    }
    while let Some(value) = data_1.pop() {
        result += (value) * data_2.iter().filter(|&n|*n == value).count() as u32;
    }
    println!("{}", result);
    Ok(())
}