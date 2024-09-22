use std::io::stdin;

fn parse_float_vector(linea: &String) -> Vec<f64> {
	let mut vector: Vec<f64> = Vec::new();
	for numero in linea.split(" ") {
		vector.push(numero.to_string().parse().unwrap());
	}
	vector
}

fn main() {
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
		matriz.push(parse_float_vector(&mut linea));
		linea.clear();
		if i > 0 {
			if matriz[i].len() != matriz[i-1].len() {
				println!("!!!");
				println!("Las filas de la matriz deben ser de igual tamaño.");
				return;
			}
		}
		i += 1;
	}
	

	println!("La entrada es: ");
	for vector in matriz {
		for valor in vector {
			print!("{valor} ");
		}
		println!();
	}
}
