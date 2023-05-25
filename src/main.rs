use std::io::{BufReader, BufRead};
use std::fs::File;

use ndarray::{Array2, Axis};


fn tick_round(energy_map: &mut Array2::<u8>, flashes: &mut usize) {

    *energy_map += 1;

    let mut mask: Array2::<u8> = Array2::ones(energy_map.dim());

    let mut flashed = true;

    while flashed {

        flashed = false;

        for row in 0..energy_map.nrows() {
            for column in 0..energy_map.ncols() {

                if energy_map[[row, column]] > 9 {

                    *flashes += 1;
                    energy_map[[row, column]] = 0;
                    mask[[row, column]] = 0;
                    flashed = true;

                    if row == 0 && column == 0 {
                        energy_map[[row + 1, column]] += 1;
                        energy_map[[row, column + 1]] += 1;
                        energy_map[[row + 1, column + 1]] += 1;
                    }
                    else if row == 0 && column == energy_map.ncols() - 1 {
                        energy_map[[row + 1, column]] += 1;
                        energy_map[[row, column - 1]] += 1;
                        energy_map[[row + 1, column - 1]] += 1;
                    }
                    else if row == 0 {
                        energy_map[[row, column - 1]] += 1;
                        energy_map[[row, column + 1]] += 1;
                        energy_map[[row + 1, column - 1]] += 1;
                        energy_map[[row + 1, column]] += 1;
                        energy_map[[row + 1, column + 1]] += 1;
                    }
                    else if row == energy_map.nrows() - 1 && column == 0 {
                        energy_map[[row - 1, column]] += 1;
                        energy_map[[row, column + 1]] += 1;
                        energy_map[[row - 1, column + 1]] += 1;
                    }
                    else if row == energy_map.nrows() - 1 && column == energy_map.ncols() - 1 {
                        energy_map[[row - 1, column]] += 1;
                        energy_map[[row, column - 1]] += 1;
                        energy_map[[row - 1, column - 1]] += 1;
                    }
                    else if row == energy_map.nrows() - 1 {
                        energy_map[[row - 1, column - 1]] += 1;
                        energy_map[[row - 1, column]] += 1;
                        energy_map[[row - 1, column + 1]] += 1;
                        energy_map[[row, column - 1]] += 1;
                        energy_map[[row, column + 1]] += 1;
                    } else if column == 0 {
                        energy_map[[row - 1, column + 1]] += 1;
                        energy_map[[row, column + 1]] += 1;
                        energy_map[[row + 1, column + 1]] += 1;
                        energy_map[[row - 1, column]] += 1;
                        energy_map[[row + 1, column]] += 1;               
                    } else if column == energy_map.ncols() - 1 {
                        energy_map[[row - 1, column - 1]] += 1;
                        energy_map[[row, column - 1]] += 1;
                        energy_map[[row + 1, column - 1]] += 1;
                        energy_map[[row - 1, column]] += 1;
                        energy_map[[row + 1, column]] += 1;               
                    } else {
                        energy_map[[row - 1, column - 1]] += 1;
                        energy_map[[row - 1, column]] += 1;
                        energy_map[[row - 1, column + 1]] += 1;
                        energy_map[[row, column - 1]] += 1;
                        energy_map[[row, column + 1]] += 1;
                        energy_map[[row + 1, column - 1]] += 1;
                        energy_map[[row + 1, column]] += 1;
                        energy_map[[row + 1, column + 1]] += 1;
                    }
                }
            }
        }
    
        *energy_map *= &mask;
    }
}

fn main() {
    let lines = BufReader::new(File::open("input.txt").unwrap()).lines()
    .map(|l| l.unwrap()).collect::<Vec<String>>();

    let mut energy_map= Array2::zeros((lines.len(), lines[0].len()));

    for (i, mut row) in energy_map.axis_iter_mut(Axis(0)).enumerate() {
        for (j, e) in row.iter_mut().enumerate() {
            *e = lines[i].chars().nth(j).unwrap().to_digit(10).unwrap() as u8;
        }
    }

    let mut rounds = 0;
    let mut flashes = 0;

    while energy_map != Array2::zeros(energy_map.dim()) {
        tick_round(&mut energy_map, &mut flashes);
        rounds += 1;
    }

    println!("rounds {}", rounds);
}
