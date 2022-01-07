fn main() {
    let mut line1 = String::new();
	let b1 = std::io::stdin().read_line(&mut line1).unwrap();

    let mut line2 = String::new();
	let b2 = std::io::stdin().read_line(&mut line2).unwrap();
	
	let line1 : i32 = line1.trim().parse().unwrap();
	let line2 : i32 = line2.trim().parse().unwrap();
	let resultado = line1+line2;

    println!("X = {}", resultado);
}