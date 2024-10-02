mod vec_io;
use ndarray as nd;

pub fn read_matrix(message: &str) -> nd::Array2<f64> {
	let vecvec: Vec<Vec<f64>> = vec_io::read_vecvec(message)
		.expect("Failed to read Vec<Vec<f64>>");
	let vec: Vec<f64> = vecvec.clone()
	.into_iter()
	.flatten()
	.collect();

	if !vec.is_empty() {
		nd::Array::from_shape_vec((vecvec.len(), vecvec[0].len()), vec).unwrap()
	} else {
		nd::Array2::zeros((0, 0))
	}
}

pub fn read_row(message: &str) -> nd::Array2<f64> {
	let vec = vec_io::read_vec(message);

	if !vec.is_empty() {
		nd::Array::from_shape_vec((1, vec.len()), vec).unwrap()
	} else {
		nd::Array2::zeros((0, 0))
	}
}

pub fn read_column(message: &str) -> nd::Array2<f64> {
	let vec = vec_io::read_vec(message);

	if !vec.is_empty() {
		nd::Array::from_shape_vec((vec.len(), 1), vec).unwrap()
	} else {
		nd::Array2::zeros((0, 0))
	}
}

pub fn pretty_print_array2(array: &nd::Array2<f64>) {
    for row in array.rows() {
        let row_string: Vec<String> = row.iter().map(|&x| format!("{:6.2} ", x)).collect();
        println!("[ {} ]", row_string.join(" "));
    }
}
