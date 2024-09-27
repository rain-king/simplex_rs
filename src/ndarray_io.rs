mod vec_io;
use vec_io::read_vecvec;
use vec_io::read_vec;
use ndarray as nd;
use nd::Array2;

pub fn read_matrix(message: &str) -> Array2<f64> {
	let vecvec: Vec<Vec<f64>> = read_vecvec(message).unwrap();
	let vec: Vec<f64> = vecvec.clone()
	.into_iter()
	.flatten()
	.collect();

	nd::Array::from_shape_vec((vecvec.len(), vecvec[0].len()), vec).unwrap()
}

pub fn read_row(message: &str) -> Array2<f64> {
	let vec = read_vec(message);

	nd::Array::from_shape_vec((1, vec.len()), vec).unwrap()
}

pub fn read_column(message: &str) -> Array2<f64> {
	let vec = read_vec(message);

	nd::Array::from_shape_vec((vec.len(), 1), vec).unwrap()
}

pub fn pretty_print_array2(array: &Array2<f64>) {
    for row in array.rows() {
        let row_string: Vec<String> = row.iter().map(|&x| format!("{:5.2} ", x)).collect();
        println!("[ {} ]", row_string.join(" "));
    }
}
