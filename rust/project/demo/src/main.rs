use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Hello, world!");
	let secert_number = rand::thread_rng().gen_range(1..101);
	println!("the secret is: {secert_number}");

	loop{
		println!("Please input:");

		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("error");
		
		// let guess: u32 = guess.trim().parse().expect("Please enter number!!");  //shadow
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,

		};
		println!("\nYour input: {}", guess);

		match guess.cmp(&secert_number){
			Ordering::Less => println!("too smal\n"),	//arm
			Ordering::Greater => println!("too big\n"),
			Ordering::Equal => {
				println!("Win!!\n");
				break;
			}
		}
	}
}