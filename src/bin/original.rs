use memchr::memchr;
use memmap::Mmap;
use std::fs::File;

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

    for (row, row_count) in lines.zip(&mut galaxies_in_row) {
        for (cell, column_count) in row.iter().copied().zip(&mut galaxies_in_column) {
            if cell == b'#' {
                *row_count += 1;
                *column_count += 1;
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
