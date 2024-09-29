use core::f64;
use crate::simplex_args::{A, B, Z};
use crate::ndarray_io::pretty_print_array2;
use ndarray::{concatenate, s, Array1 as vector, Array2 as matrix, Axis};

pub fn big_m_simplex(z: Z, a_matrix: A, b: B) -> Vec<(usize, usize)> {
	let mut basis: Vec<(usize, usize)> = Vec::new();
	let mut tableau = original_tableau(&z, &a_matrix, &b);
	
	println!();
	println!("The initial tableau is:");
	pretty_print_array2(&tableau);
	println!();

	tableau = initialize(&z, &a_matrix, &b);
	
	basis = iterations(&mut tableau);
	println!("The final tableau is:");
	pretty_print_array2(&tableau);
	println!();

	basis
}

fn original_tableau(z: &Z, a_matrix: &A, b: &B) -> matrix<f64> {
	let tableau_bottom = get_tableu_bottom(a_matrix, b);

	let tableau_top = concatenate![
		Axis(1),
		if z.maximize {
			-z.c.clone()
		} else {
			z.c.clone()
		},
		matrix::zeros((1, tableau_bottom.ncols() - z.c.ncols()))
	];
	
	let tableau = concatenate![Axis(0), tableau_top, tableau_bottom];
	
	tableau
}

fn initialize(z: &Z, a: &A, b: &B) -> matrix<f64> {
	let tableau_bottom = get_tableu_bottom(a, b);
	let mut tableau_top: matrix<f64>;

	let n_geq_ineqs = b.ineq.column(0).iter().filter(|&&x| x < 0.0).count();
	let n_ineqs = a.ineq.nrows();
	let n_eqs = a.eq.nrows();

	let only_leq_constraints = n_geq_ineqs + n_eqs == 0;

	tableau_top = concatenate![
		Axis(1),
		if z.maximize {
			-z.c.clone()
		} else {
			z.c.clone()
		},
		matrix::zeros((1, tableau_bottom.ncols() - z.c.ncols()))
	];
	let mut tableau = concatenate![Axis(0), tableau_top, tableau_bottom];
	
	let big_m = tableau.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()*10.0;

	if !only_leq_constraints {
		let artificials_column_index_start = tableau_top.ncols() - 1 - (n_geq_ineqs + a.eq.nrows());
		let tableau_top_ncols = tableau_top.ncols();
		tableau_top
			.columns_mut()
			.into_iter()
			.enumerate()
			.filter(|(j, _)| artificials_column_index_start <= *j && *j < tableau_top_ncols - 1)
			.for_each(|(_, column)|
				for value in column {
					*value = if z.maximize {-big_m} else {big_m};
				}
			);
			tableau = concatenate![Axis(0), tableau_top, tableau_bottom];
	}
	
	if n_geq_ineqs > 0 {
		// convert >= constraints into <= constraints
		tableau.rows_mut().into_iter()
			.filter(|row| row[row.len() - 1] < 0.0)
			.for_each(|row|
				for value in row {
					*value *= -1.0;
				}
			);
	}

	if !only_leq_constraints { // there are artificials
		let pivot_row_range: std::ops::Range<usize>;
		let mut pivot_vec: Vec<usize> = Vec::new();
		if a.ineq.is_empty() {
			// only equality artificials, pivot second to last row
			pivot_row_range = 1..(n_eqs+1);
		} else if a.eq.is_empty() {
			// only >= constraints
			let n_geq_ineqs = b.ineq.column(0).iter().filter(|&&value| value < 0.0).count();
			pivot_row_range = 1..(n_geq_ineqs+1);
		} else {
			pivot_row_range = (n_ineqs+1)..tableau.nrows();
			pivot_vec = b.ineq.column(0).into_iter()
					.enumerate()
					.filter(|(_, &x)| x < 0.0)
					.map(|(i, _)| i+1)
					.collect();
		}

		for i in pivot_row_range.chain(pivot_vec) {
			let pivot_row = tableau.row(i).to_owned();
			tableau
				.row_mut(0)
				.iter_mut()
				.zip(pivot_row.iter())
				.for_each(|(value, &pivot_value)| *value += if z.maximize {pivot_value*big_m} else {-pivot_value*big_m});
		}
	}
	
	// dbg!(&tableau.row(0));
	// panic!();
	
	tableau
}

fn get_tableu_bottom(a_matrix: &A, b: &B) -> matrix<f64> {
	let stacked_a_matrix: matrix<f64>;
	let stacked_b: matrix<f64>;
	let a_and_slacks_and_eq_arts: matrix<f64>;
	let slacks_and_eq_arts = matrix::eye(a_matrix.ineq.nrows() + a_matrix.eq.nrows()); // slacks plus eq artificials
	let mut geq_arts = matrix::zeros((b.ineq.nrows(), 0));

	if a_matrix.eq.is_empty() {
		stacked_a_matrix = a_matrix.ineq.to_owned();
		stacked_b = b.ineq.to_owned();
		a_and_slacks_and_eq_arts = concatenate![Axis(1), stacked_a_matrix, slacks_and_eq_arts];
		geq_arts = get_geq_artificials(b);
	} else if a_matrix.ineq.is_empty() {
		stacked_a_matrix = a_matrix.eq.to_owned();
		stacked_b = b.eq.to_owned();
		a_and_slacks_and_eq_arts = concatenate![Axis(1), stacked_a_matrix, slacks_and_eq_arts];
	} else {
		stacked_a_matrix = concatenate![Axis(0), a_matrix.ineq, a_matrix.eq];
		stacked_b = concatenate![Axis(0), b.ineq, b.eq];
		a_and_slacks_and_eq_arts = concatenate![Axis(1), stacked_a_matrix, slacks_and_eq_arts];
		geq_arts = get_geq_artificials(b);
	}

	let stacked_geq_artificials = concatenate![
		Axis(0),
		geq_arts,
		matrix::zeros((a_matrix.eq.nrows(), geq_arts.ncols()))
	];

	let a_slacks_geq = concatenate![Axis(1), a_and_slacks_and_eq_arts, stacked_geq_artificials];

	concatenate![Axis(1), a_slacks_geq, stacked_b]
}

fn get_geq_artificials(b: &B) -> matrix<f64> {
	let mut geq_artificials = matrix::zeros((b.ineq.nrows(), 0));
	for (i, value) in b.ineq.column(0).iter().enumerate() {
		let mut geq_artificial_column = matrix::zeros((b.ineq.nrows(), 1));
		if *value < 0.0 {
			geq_artificial_column[(i, 0)] = -1.0;
			geq_artificials = concatenate![Axis(1), geq_artificials, geq_artificial_column];
		}
	}
	geq_artificials
}

fn iterations(tableau: &mut matrix<f64>) -> Vec<(usize, usize)> {
	let mut basis = initialize_basis(tableau.to_owned());

	let mut iteration = 1;
	while not_optimal(tableau) {
		let (pivot_row_index, pivot_column_index) = pivot(tableau);
		for element in basis.iter_mut() {
			// variable with pivot column enters, variable with pivot row exits
			if element.0 == pivot_row_index {
				*element = (pivot_row_index, pivot_column_index);
			}
		}

		println!("Iteration {iteration}");
		pretty_print_array2(&tableau);
		println!();

		iteration += 1;
	}

	basis
}

fn initialize_basis(tableau: matrix<f64>) -> Vec<(usize, usize)> {
	let mut basis = Vec::new();
	for j in 0..(tableau.ncols() - 1) {
		// avoid right hand side
		let col = tableau.column(j).slice(s![1..]).to_owned();
		if is_basic(col) {
			for i in 1..tableau.nrows() {
				if tableau[(i, j)] == 1.0 {
					basis.push((i, j));
				}
			}
		}
	}

	basis
}

fn is_basic(column: vector<f64>) -> bool {
	let has_only_one_1 = column.iter().filter(|&&x| x == 1.0).count() == 1;
	let everything_else_is_0 = column.iter().filter(|&&x| x == 0.0).count() == column.len() - 1;
	has_only_one_1 && everything_else_is_0
}

fn not_optimal(tableau: &matrix<f64>) -> bool {
	tableau.row(0).slice(s![..-1]).into_iter().any(|&x| x < 0.0)
}

fn pivot(tableau: &mut matrix<f64>) -> (usize, usize) {
	let (pivot_row_index, pivot_column_index) = pivot_indexes(tableau);

	let normalization_scalar = tableau[(pivot_row_index, pivot_column_index)].to_owned();
	tableau
		.row_mut(pivot_row_index)
		.map_inplace(|x| *x /= normalization_scalar);

	let pivot_row = tableau.row(pivot_row_index).to_owned();
	for mut row in tableau.rows_mut().into_iter() {
		if row != pivot_row {
			let pivot_value = row[pivot_column_index];
			row.zip_mut_with(&pivot_row, |r, p| *r -= p * pivot_value);
		}
	}

	(pivot_row_index, pivot_column_index)
}

fn pivot_indexes(tableau: &mut matrix<f64>) -> (usize, usize) {
	let pivot_column_index = argmin(tableau.row(0).slice(s![..-1]).to_owned());

	let mut pivot_row_index = 0 as usize;
	let mut minimum = f64::INFINITY;
	for (i, pivot_value) in tableau.column(pivot_column_index).into_iter().enumerate() {
		if i > 0 {
			let right_hand_side_value = tableau[(i, tableau.ncols() - 1)];
			if *pivot_value > 0.0 {
				let quotient = right_hand_side_value / pivot_value;
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
