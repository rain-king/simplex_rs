use core::f64;
use std::vec::IntoIter;

use crate::simplex_args;
use simplex_args::{A, B, Z};

use crate::ndarray_io;
use ndarray_io::pretty_print_array2;

use nd::{concatenate, s, Array1 as vector, Array2 as matrix, Axis};
use ndarray as nd;

pub fn two_phase_simplex(z: Z, a_matrix: A, b: B) -> Vec<(usize, usize)> {
    let mut tableau: matrix<f64>;
    let to_phase_two: bool;
    let mut basis: Vec<(usize, usize)>;

    println!();
    tableau = original_tableau(&z, &a_matrix, &b);
    println!("The initial tableau is:");
    pretty_print_array2(&tableau);
    println!();

    (tableau, to_phase_two) = initialize_to_phase_one(&z, &a_matrix, &b);
    if to_phase_two {
        println!("The tableau before phase one is:");
        pretty_print_array2(&tableau);
        println!();

        basis = iterations(z.maximize, &mut tableau);
        println!("The tableau after phase one is:");
        pretty_print_array2(&tableau);
        println!();

        tableau = phase_two(tableau, z.c, basis);

        println!("The initialized tableau for phase two is:");
        pretty_print_array2(&tableau);
        println!();
    }
    basis = iterations(z.maximize, &mut tableau);
    println!("The final tableau is:");
    pretty_print_array2(&tableau);
    println!();

    basis
}

fn original_tableau(z: &Z, a_matrix: &A, b: &B) -> matrix<f64> {
    // stacking A with A_eq and b with b.eq
    let stacked_a_matrix: matrix<f64>;
    let stacked_b: matrix<f64>;
    let slacks: matrix<f64>;
    let a_slacks: matrix<f64>;
    let mut geq_artificials = matrix::zeros((b.ineq.nrows(), 0));
    if a_matrix.eq.is_empty() {
        stacked_a_matrix = a_matrix.ineq.to_owned();
        stacked_b = b.ineq.to_owned();
        slacks = matrix::eye(a_matrix.ineq.nrows() + a_matrix.eq.nrows());
        a_slacks = concatenate![Axis(1), stacked_a_matrix, slacks];
        geq_artificials = geq_artificials_pack(b);
    } else if a_matrix.ineq.is_empty() {
        stacked_a_matrix = a_matrix.eq.to_owned();
        stacked_b = b.eq.to_owned();
        slacks = matrix::eye(a_matrix.ineq.nrows() + a_matrix.eq.nrows());
        a_slacks = concatenate![Axis(1), stacked_a_matrix, slacks];
    } else {
        stacked_a_matrix = concatenate![Axis(0), a_matrix.ineq, a_matrix.eq];
        stacked_b = concatenate![Axis(0), b.ineq, b.eq];
        slacks = matrix::eye(a_matrix.ineq.nrows() + a_matrix.eq.nrows());
        a_slacks = concatenate![Axis(1), stacked_a_matrix, slacks];
        geq_artificials = geq_artificials_pack(b);
    }

    let stacked_geq_artificials = concatenate![
        Axis(0),
        geq_artificials,
        matrix::zeros((a_matrix.eq.nrows(), geq_artificials.ncols()))
    ];

    // println!("\n{}\n{}\n\n", a_slacks, geq_artificials);

    let a_slacks_geq = concatenate![Axis(1), a_slacks, stacked_geq_artificials];

    let tableau_bottom = concatenate![Axis(1), a_slacks_geq, stacked_b];

    let tableau_top = concatenate![
        Axis(1),
        if z.maximize {
            -z.c.clone()
        } else {
            z.c.clone()
        },
        matrix::zeros((1, tableau_bottom.ncols() - z.c.ncols()))
    ];
    concatenate![Axis(0), tableau_top, tableau_bottom]
}

fn initialize_to_phase_one(z: &Z, a_matrix: &A, b: &B) -> (matrix<f64>, bool) {
    // stacking A with A_eq and b with b.eq
    let stacked_a_matrix: matrix<f64>;
    let stacked_b: matrix<f64>;
    let slacks: matrix<f64>;
    let a_slacks: matrix<f64>;
    let mut geq_artificials = matrix::zeros((b.ineq.nrows(), 0));
    let geq_artificials_rows: Vec<usize> = Vec::new();

    if a_matrix.eq.is_empty() {
        stacked_a_matrix = a_matrix.ineq.to_owned();
        stacked_b = b.ineq.to_owned();
        slacks = matrix::eye(a_matrix.ineq.nrows() + a_matrix.eq.nrows());
        a_slacks = concatenate![Axis(1), stacked_a_matrix, slacks];
        geq_artificials = geq_artificials_pack(b);
    } else if a_matrix.ineq.is_empty() {
        stacked_a_matrix = a_matrix.eq.to_owned();
        stacked_b = b.eq.to_owned();
        slacks = matrix::eye(a_matrix.ineq.nrows() + a_matrix.eq.nrows());
        a_slacks = concatenate![Axis(1), stacked_a_matrix, slacks];
    } else {
        stacked_a_matrix = concatenate![Axis(0), a_matrix.ineq, a_matrix.eq];
        stacked_b = concatenate![Axis(0), b.ineq, b.eq];
        slacks = matrix::eye(a_matrix.ineq.nrows() + a_matrix.eq.nrows());
        a_slacks = concatenate![Axis(1), stacked_a_matrix, slacks];
        geq_artificials = geq_artificials_pack(b);
    }

    let stacked_geq_artificials = concatenate![
        Axis(0),
        geq_artificials,
        matrix::zeros((a_matrix.eq.nrows(), geq_artificials.ncols()))
    ];

    // println!("\n{}\n{}\n\n", a_slacks, geq_artificials);

    let a_slacks_geq = concatenate![Axis(1), a_slacks, stacked_geq_artificials];

    let tableau_bottom = concatenate![Axis(1), a_slacks_geq, stacked_b];

    let tableau_top: matrix<f64>;
    let only_leq_constraints = slacks.nrows() + a_matrix.eq.nrows() == a_matrix.ineq.nrows();
    if only_leq_constraints {
        // only <= constraints, prepare for regular simplex
        tableau_top = concatenate![
            Axis(1),
            if z.maximize {
                -z.c.clone()
            } else {
                z.c.clone()
            },
            matrix::zeros((1, tableau_bottom.ncols() - z.c.ncols()))
        ];
    } else {
        // mixed constraints, prepare top for phase two
        let mut w_top: matrix<f64> = matrix::zeros((1, tableau_bottom.ncols()));
        let w_top_ncols = a_slacks_geq.ncols();
        let artificials_column_index_start =
            w_top.ncols() - 1 - (geq_artificials.ncols() + a_matrix.eq.nrows());
        w_top
            .columns_mut()
            .into_iter()
            .enumerate()
            .filter(|(j, _)| artificials_column_index_start <= *j && *j < w_top_ncols)
            .for_each(|(_, column)| {
                for value in column {
                    *value = -1.0;
                }
            });
        tableau_top = w_top;
    };

    let mut tableau = concatenate![Axis(0), tableau_top, tableau_bottom];

    if !only_leq_constraints {
        let mut ineq_iterator: IntoIter<usize> = Vec::new().into_iter();
        let eq_iterator: std::ops::Range<usize>;
        if !a_matrix.ineq.is_empty() {
            // convert >= constraints into <= constraints
            tableau
                .rows_mut()
                .into_iter()
                .filter(|row| row[row.len() - 1] < 0.0)
                .for_each(|row| {
                    for value in row {
                        *value *= -1.0;
                    }
                });
            // add artificials to base
            ineq_iterator = geq_artificials_rows.into_iter();
        }
        // add equality artificials to base
        if !a_matrix.ineq.is_empty() {
            eq_iterator = (a_matrix.ineq.nrows() + 1)..(stacked_a_matrix.nrows() + 1);
        } else {
            eq_iterator = 1..(a_matrix.eq.nrows() + 1);
        }

        // println!("\n{:?}\n\n{:?}\n", ineq_iterator, eq_iterator);

        for i in ineq_iterator.chain(eq_iterator) {
            let pivot_row = tableau.row(i).to_owned();
            tableau
                .row_mut(0)
                .iter_mut()
                .zip(pivot_row.iter())
                .for_each(|(value, &pivot_value)| *value += pivot_value);
        }
        println!("Before phase 1 simplex pass.");
        pretty_print_array2(&tableau);
        println!();
    }

    (tableau, !only_leq_constraints)
}

fn phase_two(tableau: matrix<f64>, c: matrix<f64>, basis: Vec<(usize, usize)>) -> matrix<f64> {
    let z_top = concatenate![
        Axis(1),
        -c.to_owned(),
        matrix::zeros((1, tableau.ncols() - c.ncols()))
    ];

    let tableau_phase_two = concatenate![Axis(0), z_top, tableau.slice(s![1.., ..])];

    let basis_cols: Vec<usize> = basis.iter().map(|&x| x.1).collect();

    let phase_two_columns: Vec<Vec<f64>> = tableau_phase_two
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(j, _)| {
            (0..c.ncols()).contains(j) || basis_cols.contains(j) || *j == (tableau.ncols() - 1)
        })
        .map(|(_, column)| column.to_owned().to_vec())
        .collect();

    // SHADOWING
    let mut tableau_phase_two = matrix::zeros((tableau.nrows(), 0));

    for vec_column in phase_two_columns {
        let column = matrix::from_shape_vec((tableau.nrows(), 1), vec_column).unwrap();
        tableau_phase_two = concatenate![Axis(1), tableau_phase_two, column]
    }

    // pretty_print_array2(&tableau_phase_two);
    // println!();

    let basis_phase_two: Vec<(usize, usize)> = basis
        .iter()
        .filter(|element| element.1 < c.ncols())
        .map(|x| *x)
        .collect();

    for (pivot_row_i, pivot_column_i) in basis_phase_two {
        let pivot_row = tableau_phase_two.row(pivot_row_i).to_owned();
        let to_be_deleted_value = tableau_phase_two.row(0)[pivot_column_i];
        tableau_phase_two
            .row_mut(0)
            .iter_mut()
            .zip(pivot_row.into_iter())
            .for_each(|(value, pivot_value)| *value -= pivot_value * to_be_deleted_value);
    }

    println!("Before phase two");
    pretty_print_array2(&tableau_phase_two);
    println!();

    tableau_phase_two
}

fn geq_artificials_pack(b: &B) -> matrix<f64> {
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

// THIS CODE SUPPOSES THE TABLEAU IS IN BASIC FORM
fn iterations(maximize: bool, tableau: &mut matrix<f64>) -> Vec<(usize, usize)> {
    // create initial basis from basic form
    let mut basis = initialize_basis(tableau.to_owned());

    // dbg!(&basis);
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
