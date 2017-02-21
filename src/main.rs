fn main() {
	/*let x: i64 = 5;
	let a: i64 = 8;
	let sum = x + a;

	{
		println!("{:?}", a);
		let a = 12;
		println!("{:?}", a);
	}

	println!("{:?}", a);
	println!("{}", sum);*/

	// print_number(77);
	// print_sum(9, 8);
	println!("{:?}", add_one(9));;
}

fn print_number(x: i64) {
	println!("{:?}", x);
}

fn print_sum(x: i64, y: i64) {
	println!("{:?}", x + y);
}

fn add_one(x: i64) -> i64 {
	x + 1
}