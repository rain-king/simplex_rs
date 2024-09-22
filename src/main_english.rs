use std::io::stdin;

fn parse_float_vector(numbers: &String) -> Vec<f64> {
	let mut vector: Vec<f64> = Vec::new();
	for number in numbers.split(" ") {
		vector.push(number.parse().unwrap());
	}
	vector
}

fn main() {
	let mut matrix: Vec<Vec<f64>> = Vec::new();
	let mut line: String = String::new();

	println!("Enter the decimal numbers of each row line by line");
	println!("separated by spaces, and when done hit Enter again:");
	
	let mut i: usize = 0;
	loop {
		stdin().read_line(&mut line).unwrap();
		line = line.trim().to_string();
		if line.is_empty() {
			break;
		}
		matrix.push(parse_float_vector(&mut line));
		line.clear();
		if i > 0 {
			if matrix[i].len() != matrix[i-1].len() {
				println!("!!!");
				println!("The rows of the matrix must be of equal size.");
				return;
			}
		}
		i += 1;
	}

	println!("The input matrix is: ");
	for vector in matrix {
		for value in vector {
			print!("{value} ");
		}
		println!();
	}
}
