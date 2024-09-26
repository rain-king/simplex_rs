use std::io::stdin;

pub fn parse_float_vector(line: &str) -> Vec<f64> {
	let vector: Vec<f64> = line
		.split_whitespace()
		.map(|x| x.parse().unwrap())
		.collect();
	vector
}

pub fn read_vector(message: &str) -> Vec<f64> {
	println!("{message}");

	let mut line: String = String::new();
	stdin().read_line(&mut line).unwrap();

	let mut vector: Vec<f64> = Vec::new();
	vector = line.split_whitespace()
		.map(|x| x.parse().unwrap())
		.collect();
	vector
}

pub fn read_matrix(message: &str) -> Result<Vec<Vec<f64>>, String> {
	let mut matrix: Vec<Vec<f64>> = Vec::new();
	let mut line: String = String::new();

	if message.is_empty() {
		println!("Enter the values of the matrix row by row");
		println!("separated by spaces, then press Enter again:");	
	} else {
		print!("{message}");
	}
	
	let mut i: usize = 0;
	loop {
		stdin().read_line(&mut line).unwrap();
		line = line.trim().to_string();
		if line.is_empty() {
			break;
		}
		matrix.push(parse_float_vector(&line));
		line.clear();
		if i > 0 && matrix[i].len() != matrix[i-1].len() {
  				return Err("The matrix rows should be of equal size.".to_string());
  			}
		i += 1;
	}
	Ok(matrix)
}

pub fn print_matrix(matrix: Vec<Vec<f64>>) {
	for vector in matrix {
		for valor in vector {
			print!("{valor}\t");
		}
		println!();
	}
}
