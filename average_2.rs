fn main() {
	let A = input();
	let B = input();
	let C = input();

	println!("MEDIA = {:.2}",  mean(A,B,C));
}

fn mean(x:f64, y:f64, z:f64) -> f64{
	((x*2.0)+(y*3.0)+(z*5.0))/10.0
}

fn input() -> f64 {
	let mut A = String::new();
	let A1 = std::io::stdin().read_line(&mut A).unwrap();
	let A_float : f64 = A.trim().parse().unwrap();
	if A_float > 10.0 ||  A.chars().count() > 4 {
		let A = A.clear();
		let A = input();
		return A;
	}
	A_float
}

