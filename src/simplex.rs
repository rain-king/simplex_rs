use core::f64;

use crate::ndarray_io;
use ndarray_io::pretty_print_array2;

use ndarray as nd;
use nd::{Axis, concatenate, Array2 as matrix, Array1 as vector, s};

pub fn max_simplex
(
	maximize: bool,
	c: matrix<f64>,
	a_matrix: matrix<f64>,
	b: matrix<f64>,
	eq_matrix: matrix<f64>,
	b_eq: matrix<f64>
) -> Vec<(usize, usize)> 
{
	// println!("a_matrix\n{}\nb\n{}\neq_matrix\n{}\nb_eq\n{}\n", a_matrix, b, eq_matrix, b_eq);
	// tableau initialization	
	let mut tableau: matrix<f64>;
	if a_matrix.nrows() == 0 {
		tableau = initialize_without_ineq(maximize, c, eq_matrix, b_eq);
	} else if eq_matrix.nrows() == 0 {
		tableau = initialize_without_eq(maximize, c, a_matrix, b);
	} else {
		tableau = initialize_with_both(maximize, c, a_matrix, b, eq_matrix, b_eq);
	};

	// tableau = phase_two(maximize, c, &mut tableau);

	// println!("The initial tableau is:");
	// pretty_print_array2(&tableau);
	// println!();

	// let basis = iterations(&mut tableau, maximize);

	// println!("The final tableau is:");
	// pretty_print_array2(&tableau);

	basis
}

fn initialize_without_ineq(
	maximize: bool,
	c: matrix<f64>,
	eq_matrix: matrix<f64>,
	b_eq: matrix<f64>
) -> matrix<f64>
{	
	let slacks: matrix<f64> = matrix::eye(eq_matrix.nrows());

	let eq_matrix_slacks = concatenate![Axis(1), eq_matrix, slacks];

	let tableau_bottom = concatenate![
		Axis(1),
		eq_matrix_slacks,
		b_eq
	];

	let tableau_top = concatenate![
		Axis(1),
		if maximize { -c.clone() } else { c.clone() },
		matrix::zeros((1, tableau_bottom.ncols() - c.ncols()))
	];

	concatenate![Axis(0), tableau_top, tableau_bottom]
}

fn initialize_without_eq(
	maximize: bool,
	c: matrix<f64>,
	a_matrix: matrix<f64>,
	b: matrix<f64>
) -> matrix<f64>
{	
	let slacks: matrix<f64> = matrix::eye(a_matrix.nrows());

	let eq_matrix_slacks = concatenate![Axis(1), a_matrix, slacks];

	let tableau_bottom = concatenate![
		Axis(1),
		eq_matrix_slacks,
		b
	];

	let tableau_top = concatenate![
		Axis(1),
		if maximize { -c.clone() } else { c.clone() },
		matrix::zeros((1, tableau_bottom.ncols() - c.ncols()))
	];

	concatenate![Axis(0), tableau_top, tableau_bottom]
}

fn initialize_with_both
(
	maximize: bool,
	c: matrix<f64>,
	a_matrix: matrix<f64>,
	b: matrix<f64>,
	eq_matrix: matrix<f64>,
	b_eq: matrix<f64>
) -> matrix<f64>
{		
	// stacking A with A_eq and b with b_eq
	let stacked_a_matrix = concatenate![Axis(0), a_matrix, eq_matrix];
	let stacked_b = concatenate![Axis(0), b, b_eq];

	// println!("stacked_a\n{}\n{}\n", stacked_a_matrix, stacked_b);
	// stacking horizontally

	// get slacks for all inequalities and artificials for equalities
	let slacks: matrix<f64> = matrix::eye(a_matrix.nrows() + eq_matrix.nrows());

	let a_slacks = concatenate![Axis(1), stacked_a_matrix, slacks];

	// get artificial variables for >= constraints
	let geq_count = b.rows().into_iter().filter(|x| x[0] < 0.0).count();
	let mut geq_artificials = matrix::zeros((b.nrows(), 0));
	for (i, value) in b.column(0).iter().enumerate() {
		let mut geq_artificial_column = matrix::zeros((b.nrows(), 1));
		if *value < 0.0 {
			geq_artificial_column[(i, 0)] = -1.0;
			geq_artificials = concatenate![Axis(1), geq_artificials, geq_artificial_column];
	}
	}

	let stacked_geq_artificials = concatenate![
		Axis(0),
		geq_artificials,
		matrix::zeros((eq_matrix.nrows(), geq_artificials.ncols()))
	];

	println!("\n{}\n{}\n\n", a_slacks, geq_artificials);

	let a_slacks_geq = concatenate![
		Axis(1),
		a_slacks,
		stacked_geq_artificials
	];

	let tableau_bottom = concatenate![
		Axis(1),
		a_slacks_geq,
		stacked_b
	];
	let tableau_top = concatenate![
		Axis(1),
		if maximize { -c.clone() } else { c.clone() },
		matrix::zeros((1, tableau_bottom.ncols() - c.ncols()))
	];

	let mut tableau = concatenate![Axis(0), tableau_top, tableau_bottom];

	// finally, we need to convert the >= inequalities into <= inequalities
	tableau.rows_mut()
		.into_iter()
		.filter(|row| row[row.len() - 1] < 0.0)
		.for_each(|row| for value in row {
			*value *= -1.0;
		});
	
	tableau
}

// THIS CODE SUPPOSES THE TABLEAU IS IN BASIC FORM
fn iterations(tableau: &mut matrix<f64>, maximize: bool) -> Vec<(usize, usize)> {
	// create initial basis from basic form
	let mut basis = initialize_basis(tableau.to_owned());

	// println!("Initial basis:\n{:?}", basis);

	while not_optimal(tableau, maximize) {
		let (pivot_row_index, pivot_column_index) = pivot(tableau, maximize);
		// println!("The pivot indexes are: {pivot_row_index} {pivot_column_index}:");
		for element in basis.iter_mut() {
			// variable with pivot column enters, variable with pivot row exits
			if element.0 == pivot_row_index {
				// println!("{:?}", basis);
				*element = (pivot_row_index, pivot_column_index);
			}
		}
		// println!("The current basis indexes are\n{:?}", basis);
		// pretty_print_array2(&tableau);
		// println!("");
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

fn not_optimal(tableau: &mut matrix<f64>, maximize: bool) -> bool {
	if maximize {
		tableau.row(0).slice(s![..-1]).into_iter().any(|&x| x < 0.0)
	} else {
		tableau.row(0).slice(s![..-1]).into_iter().any(|&x| x > 0.0)
	}
}

fn pivot(tableau: &mut matrix<f64>, maximize: bool) -> (usize, usize) {
	let (pivot_row_index, pivot_column_index) = pivot_indexes(tableau, maximize);

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

fn pivot_indexes(tableau: &mut matrix<f64>, maximize: bool) -> (usize, usize) {
	let pivot_column_index = if maximize {
		argmin(tableau.row(0).slice(s![..-1]).to_owned())
	} else {
		argmax(tableau.row(0).slice(s![..-1]).to_owned())
	};
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

fn argmax(arr: vector<f64>) -> usize {
	let mut max = -f64::INFINITY;
	let mut argmax: usize = 0;

	for (i, value) in arr.into_iter().enumerate() {
		if value > max {
			max = value;
			argmax = i;
		}
	}
	argmax
}