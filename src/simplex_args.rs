use ndarray as nd;

pub struct Z {
	pub maximize: bool,
	pub c: nd::Array2<f64>,
}

pub struct A {
	pub ineq: nd::Array2<f64>,
	pub eq: nd::Array2<f64>,
}

pub struct B {
	pub ineq: nd::Array2<f64>,
	pub eq: nd::Array2<f64>
}