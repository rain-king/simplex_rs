use core::f64;
use std::f32::EPSILON;

use ndarray as nd;
use nd::{Axis, concatenate, Array2 as matrix, Array1 as vector};

pub fn max_simplex(c: matrix<f64>, a_matrix: matrix<f64>, b: matrix<f64>)
	// -> Vec<i64>
{
	let slacks: matrix<f64> = matrix::eye(a_matrix.nrows());

	assert_eq!(a_matrix.nrows(), slacks.nrows(), "Row counts do not match");

	let mut tableu_restrictions: matrix<f64> = concatenate![Axis(1), a_matrix, slacks];

	tableu_restrictions = concatenate![Axis(1), tableu_restrictions, b];

	// println!("{:?}", c);
	// println!("{:?}", matrix::<f64>::zeros((1,1)));

	let z_row = concatenate![Axis(1), -c.clone(), matrix::<f64>::zeros((1,tableu_restrictions.ncols()-c.ncols()))];

	let mut tableu = concatenate![Axis(0), z_row, tableu_restrictions];

	println!("{:?}", tableu);
}

// THIS CODE SUPPOSES THE TABLEU IS IN BASIC FORM
fn iteration(tableu: matrix<f64>) -> Vec<(usize, usize)> {
	let mut basis = Vec::new();

	for j in 0..(tableu.ncols()-1) {
		let col = tableu.column(j);
		for i in 1..tableu.nrows() {
			let col_sum: f64 = col.iter().sum();
			if (col_sum == 1.0) && ((tableu[(i,j)] - 1.0).abs() < f64::EPSILON)  {
				basis.push((i, j));
			}
		}
	}

	basis
}

fn pivot(tableu: matrix<f64>) -> (usize, usize) {
	let pivot_column = argmin(tableu.row(0).to_owned());
	let pivot_row = 0 as usize;

	(pivot_column, pivot_row)
}

fn argmin(arr: vector<f64>) -> usize {
	let mut min = f64::INFINITY;
	let mut argmin: usize = 0;

	for (i, value) in arr.into_iter().enumerate() {
		if value < min {
			min = value;
			argmin = i;
		}
	}
	argmin
}