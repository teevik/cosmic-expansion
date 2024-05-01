#![feature(portable_simd)]

use crossterm::style::Stylize;
use std::{
    fs,
    simd::{num::SimdUint, u32x64, u8x64},
    time::Instant,
};

fn sum_shortest_paths<const EXPANSION: u128>(input: &str) -> u128 {
    let width = input.lines().next().unwrap().len();
    let width_with_newline = width + 1;
    let height = input.lines().count();

    let mut galaxies_in_row = vec![0u32; height];
    let mut galaxies_in_column = vec![0u32; width];

    let data = input.as_bytes();

    let lines = data.chunks_exact(width_with_newline);

    let only_empty = u8x64::splat(b'.');
    let only_galaxies = u8x64::splat(b'#');

    for (row, row_count) in lines.zip(&mut galaxies_in_row) {
        const LANES: usize = u8x64::LEN;

        let chunks = row.chunks_exact(LANES);
        let chunks_amount = chunks.len();
        let remainder = chunks.remainder();

        for (chunk_x, chunk) in chunks.enumerate() {
            let row = u8x64::from_slice(chunk);

            let non_empty = row ^ only_empty;
            let galaxies = non_empty & only_galaxies;

            // Row
            let sum = galaxies.reduce_sum();
            *row_count += sum as u32;

            // Column
            let column_range = chunk_x * LANES..(chunk_x + 1) * LANES;

            let target_column = &galaxies_in_column[column_range.clone()];
            let mut target_column = u32x64::from_slice(target_column);

            target_column += galaxies.cast();

            target_column.copy_to_slice(&mut galaxies_in_column[column_range]);
        }

        for (cell, col_count) in remainder
            .into_iter()
            .zip(&mut galaxies_in_column[(chunks_amount * LANES)..])
        {
            if *cell == b'#' {
                *row_count += 1;
                *col_count += 1;
            }
        }
    }

    let mut answer = 0;

    for counts in [galaxies_in_row, galaxies_in_column] {
        // Stores the number of galaxies above the current row or column
        let mut above = 0u128;
        // Stores total distance that is influenced by the number of galaxies above
        let mut distance = 0u128;

        // Iterate through row or column
        for count in counts {
            let count = count as u128;

            if count == 0 {
                // Expand on empty rows or columns
                distance += above * EXPANSION;
            } else {
                // Update distances
                answer += distance * count;
                above += count;
                distance += above;
            }
        }
    }

    answer
}

fn read_data_file(file_name: &str) -> String {
    let path = "./data/".to_owned() + file_name;

    let mut data = fs::read_to_string(&path).expect(&format!("Failed to read file at: {}", path));

    // Add trailing newline if it's missing, and the algorithm relies on it
    if !data.ends_with('\n') {
        data.push('\n');
    }

    data
}

fn main() {
    let inputs = [
        ("original", "input.txt"),
        ("10k", "10k.txt"),
        ("50k", "50k.txt"),
        ("100k", "100k.txt"),
        ("500k", "500k.txt"),
        ("1m", "1m.txt"),
        ("10m", "10m.txt"),
    ];

    println!("{}", "Results:".bold().blue());

    let mut times_used = Vec::new();

    // Run the algorithm on each input and print the result, store the time used
    for (name, file_name) in inputs {
        let input = read_data_file(file_name);

        let start_time = Instant::now();
        let result = sum_shortest_paths::<2>(&input);
        let time_used = start_time.elapsed();

        println!("{name:10}: {result}");
        times_used.push((name, time_used));
    }

    println!();
    println!("{}", "Time used:".bold().blue());

    for (name, time_used) in times_used {
        println!("{name:10}: {time_used:?}");
    }
}
