// #![allow(warnings)]
mod simplex_args;
mod ndarray_io;
mod simplex;
use simplex_args::{Z, A, B};
use ndarray_io as io;
use simplex::big_m_simplex;
use std::io::stdin;
use std::io::BufRead; // needed to read empty lines without storing them

fn main() {
	println!("This is a program that solves the maximization or minimization of");
	println!("Z = cx subject to Ax <= b, A_eq = b_eq, with free b and b_eq,");
	println!("but x >= 0.");
	println!();

	println!("{}\n{}", "Write 1 if this is a maximization problem,",
		"or 0 if it's a minimization problem, then press Return again.");
	let maximize = read_bool();
	stdin().lock().lines().next(); // read empty line

	let c = io::read_row("Enter the c vector values separated by spaces, then press return again.");
	stdin().lock().lines().next(); // read empty line

	let a_matrix = io::read_matrix(&format!("{}{}",
		 "Enter the A matrix values row by row, with values separated by spaces,\n",
		 "then press Return again."));

	let b = io::read_column("Enter the b column values separated by spaces, then press return again.");
	stdin().lock().lines().next();

	let eq_matrix = io::read_matrix(&format!("{}{}",
	"Enter the A_eq matrix values row by row, with values separated by spaces,\n",
	"then press Return again."));

	let b_eq = io::read_column("Enter the b_eq column values separated by spaces.");

	let z = Z {
		maximize,
		c
	};
	let a_matrix = A {
		ineq: a_matrix,
		eq: eq_matrix
	};
	let b = B {
		ineq: b,
		eq: b_eq
	};

	big_m_simplex(z, a_matrix, b);
}

fn read_bool() -> bool {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	if input.trim() == "1" {
		true
	} else {
		false
	}
}
