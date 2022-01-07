fn main() {
    let mut line1 = String::new();
	let b1 = std::io::stdin().read_line(&mut line1).unwrap();
	let R : f64 = line1.trim().parse().unwrap();

	let n : f64 = 3.14159;

	let A : f64 = n*R*R;

    println!("A={:.4}", A);
}
