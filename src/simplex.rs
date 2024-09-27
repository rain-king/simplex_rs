use core::f64;

use crate::ndarray_io;
use ndarray_io::pretty_print_array2;

use ndarray as nd;
use nd::{Axis, concatenate, Array2 as matrix, Array1 as vector, s};

pub fn max_simplex(c: matrix<f64>, a_matrix: matrix<f64>, b: matrix<f64>)
	-> Vec<(usize, usize)>
{	// tableau initialization
	let slacks: matrix<f64> = matrix::eye(a_matrix.nrows());

	assert_eq!(a_matrix.nrows(), slacks.nrows(), "Row counts do not match");

	let mut tableau_restrictions: matrix<f64> = concatenate![Axis(1), a_matrix, slacks];

	tableau_restrictions = concatenate![Axis(1), tableau_restrictions, b];

	// println!("{:?}", c);
	// println!("{:?}", matrix::<f64>::zeros((1,1)));

	let z_row = concatenate![Axis(1), -c.clone(), matrix::<f64>::zeros((1,tableau_restrictions.ncols()-c.ncols()))];

	let mut tableau = concatenate![Axis(0), z_row, tableau_restrictions];

	println!("The initial tableau is:");
	pretty_print_array2(&tableau);
	println!();

	let basis = iterations(&mut tableau);

	println!("The final tableau is:");
	pretty_print_array2(&tableau);

	basis
}

// THIS CODE SUPPOSES THE TABLEAU IS IN BASIC FORM
fn iterations(tableau: &mut matrix<f64>) -> Vec<(usize, usize)> {
	// create initial basis from basic form
	let mut basis = initialize_basis(tableau.to_owned());

	println!("Initial basis:\n{:?}", basis);

	while tableau.row(0).slice(s![..-1]).into_iter().any(|&x| x < 0.0) {
		let (pivot_row_index, pivot_column_index) = pivot(tableau);
		// println!("The pivot indexes are: {pivot_row_index} {pivot_column_index}:");
		for i in 0..basis.len() {
			// variable with pivot column enters, variable with pivot row exits
			if basis[i].0 == pivot_row_index {
				// println!("{:?}", basis);
				basis[i] = (pivot_row_index, pivot_column_index);
			}
		}
		println!("The current basis indexes are\n{:?}", basis);
		// pretty_print_array2(&tableau);
		println!("");
	}

	basis
}

fn initialize_basis(tableau: matrix<f64>) -> Vec<(usize, usize)> {
	let mut basis = Vec::new();
	for j in 0..(tableau.ncols()-1) { // avoid right hand side
		let col = tableau.column(j).slice(s![1..]).to_owned();
		let has_only_one_1 = col.iter().filter(|&&x| x == 1.0).count() == 1;
		let everything_else_is_0 = col.iter().filter(|&&x| x == 0.0).count() == col.len() - 1;
		if has_only_one_1 && everything_else_is_0 {
			for i in 1..tableau.nrows() {
				if tableau[(i,j)] == 1.0 {
					basis.push((i,j));
				}
			}
		}
	}

	basis
}

fn pivot(tableau: &mut matrix<f64>) -> (usize, usize) {
	let (pivot_row_index, pivot_column_index) = pivot_indexes(tableau);

	let normalization_scalar = tableau[(pivot_row_index, pivot_column_index)].to_owned();
	tableau.row_mut(pivot_row_index).map_inplace(|x| *x /= normalization_scalar);

	let pivot_row = tableau.row(pivot_row_index).to_owned();
	for mut row in tableau.rows_mut().into_iter() {
		if row != pivot_row {
			let pivot_value = row[pivot_column_index];
			row.zip_mut_with(&pivot_row, |r, p| *r -= p*pivot_value);
		}
	}

	(pivot_row_index, pivot_column_index)
}

fn pivot_indexes(tableau: &mut matrix<f64>) -> (usize, usize) {
	let pivot_column_index = argmin(tableau.row(0).slice(s![..-1]).to_owned());
	// let quotients: Vec<f64> = Vec::new();
	let mut pivot_row_index = 0 as usize;
	let mut minimum = f64::INFINITY;
	for (i, pivot_value) in tableau.column(pivot_column_index).into_iter().enumerate() {
		// minimize quotients
		if i > 0 {
			let right_hand_side_value = tableau[(i, tableau.ncols() - 1)];
			if *pivot_value > 0.0 {
				let quotient = right_hand_side_value/pivot_value;
				if quotient < minimum {
					minimum = quotient;
					pivot_row_index = i;
				}
			}
		}
	}

	(pivot_row_index, pivot_column_index)
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