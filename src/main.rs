use std::io::stdin;

fn parse_float_vector(linea: &str) -> Vec<f64> {
	let mut vector: Vec<f64> = Vec::new();
	for numero in linea.split_whitespace() {
		vector.push(numero.parse().unwrap());
	}
	vector
}

fn read_matrix() -> Result<Vec<Vec<f64>>, String> {
	let mut matriz: Vec<Vec<f64>> = Vec::new();
	let mut linea: String = String::new();

	println!("Introduzca los valores de las filas una por una");
	println!("separados por espacios, y al terminar pulse enter:");
	
	let mut i: usize = 0;
	loop {
		stdin().read_line(&mut linea).unwrap();
		linea = linea.trim().to_string();
		if linea.is_empty() {
			break;
		}
		matriz.push(parse_float_vector(&linea));
		linea.clear();
		if i > 0 {
			if matriz[i].len() != matriz[i-1].len() {
				return Err(format!("Las filas de la matriz deben ser de igual tamaño."));
			}
		}
		i += 1;
	}
	Ok(matriz)
}

fn main() {
	let matriz: Vec<Vec<f64>> = read_matrix().unwrap();

	println!("La matriz es: ");
	for vector in matriz {
		for valor in vector {
			print!("{valor}\t");
		}
		println!();
	}
}
