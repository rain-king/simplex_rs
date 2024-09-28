use ndarray as nd;
use nd::Array1;

pub fn add_to_vector(modified_vector: &mut Array1<f64>, summing_vector: Array1<f64>) {
	*modified_vector = modified_vector.iter()
		.zip(summing_vector.iter())
		.map(|(x, y)| x + y)
		.collect();
}

pub fn scale_vector(vector: &mut Array1<f64>, scalar: f64) {
	let scaled: Array1<f64> = vector.iter()
		.map(|x| x*scalar)
		.collect();

	vector.assign(&scaled);
}