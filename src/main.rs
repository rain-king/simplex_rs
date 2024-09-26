mod matrix_io;
use matrix_io::read_vector;
use std::io::stdin;
use std::io::{self, BufRead}; // needed to read empty lines without storing them

extern crate nalgebra as na;
use na::{DMatrix, RowDVector};

fn main() {
	println!("This is a program that solves only the maximize");
	println!("Z = cx subject to Ax <= b with b >= 0.");
	println!();

	let mut row =
		read_vector("Enter the c vector values separated by spaces, then press return again.");
	stdin().lock().lines().next(); // read empty line
	let c = RowDVector::from_vec(row);
	
	println!("Write the A matrix row by row, with values separated by spaces,");
	println!("then press Return again.");
	let mut a_rows = Vec::new();
	loop {
		row = read_vector("");
		if row.is_empty() {
			break;
		}
		a_rows.push(RowDVector::from_vec(row));
	}

	let a_matrix = DMatrix::from_rows(&a_rows);

	println!("{c}");
	println!("{a_matrix}");

}
