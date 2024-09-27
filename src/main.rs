// #![allow(warnings)]

mod ndarray_io;
use ndarray_io as io;

mod simplex;
use simplex::max_simplex;

use std::io::stdin;
use std::io::BufRead; // needed to read empty lines without storing them


fn main() {
	println!("This is a program that solves the maximization of");
	println!("Z = cx subject to Ax <= b with b >= 0.");
	println!();

	let c = io::read_row("Enter the c vector values separated by spaces, then press return again.");
	stdin().lock().lines().next(); // read empty line
	
	let a_matrix = io::read_matrix("Write the A matrix row by row, with values separated by spaces, then press Return again.");

	let b = io::read_column("Enter the b vector values separated by spaces.");
		

	max_simplex(c, a_matrix, b);
}
