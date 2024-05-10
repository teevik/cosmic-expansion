#![feature(portable_simd)]

use memchr::memchr;
use memmap::Mmap;
use std::{
    fs::File,
    simd::{num::SimdUint, u32x64, u8x64},
};

const EXPANSION: u128 = 2;

fn solve() -> u128 {
    let file = File::open("data/10m.txt").expect("open file");
    let data = unsafe { Mmap::map(&file).expect("mmap file") };

    let width = memchr(b'\n', &data).expect("find newline");
    let width_with_newline = width + 1;
    let height = data.len() / width_with_newline + 1; // `+ 1`` because the last line doesn't have a trailing newline

    let mut galaxies_in_row = vec![0u32; height];
    let mut galaxies_in_column = vec![0u32; width];

    let lines = data.chunks(width_with_newline);

    let only_empty = u8x64::splat(b'.');
    let only_galaxies = u8x64::splat(b'#');

    for (row, row_count) in lines.zip(&mut galaxies_in_row) {
        const LANES: usize = u8x64::LEN;

        let chunks = row.chunks_exact(LANES);
        let chunks_amount = chunks.len();
        let remainder = chunks.remainder();

        // Process the chunks
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

        // Process the remainder
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

fn main() {
    let start = std::time::Instant::now();
    let result = solve();
    let elapsed = start.elapsed();

    println!("Elapsed: {elapsed:?}",);
    println!("Result: {result}");
}
