// use std::io;
// use std::f64::consts::PI; //float f64 f32
// use std::f32::MAX_10_EXP;
// use std::u32::MAX; // unsign int
// use std::i32::MIN; //int 
// use std::cmp::Ordering;
// use rand::Rng;

fn main() {
	println!("Hello, world!\n");

	// type_fun();
	// guess_fun();
	// println!("2+5={}",plus_five(2));
	a_fun();
}

fn a_fun(){
	let x = 5;
	println!("x = {x}");
	let y = {
		let x = 3;	//shadow
		x + 3
	};
	println!("\"x+3\"value = {}", y);
	// can use match
	if y % 4 == 0{
		println!("y is divisable by 4")
	} else if y % 2 == 0{
		println!("y is divisable by 2")	
	} else if y % 3 == 0{
		println!("y is divisable by 3")	
	}else {
		println!("y isn't divisable by 2, 3, 4")
	}
	let number = if false {5} else {6};
	println!("the vlaue of number is: {number}");

	let mut cnt0 = 0;
	let loop_result = loop {
		cnt0 += 1;
		if cnt0 >= 10 {
			break {
				println!("cnt0 = {cnt0}");
				cnt0 *= 10;
				cnt0 * 10
			};
		}
	}; 
	println!("loop_result * 10 = {loop_result}");
	
	cnt0 = 0;
	print!("\nwhile loop:");
	while  cnt0 < 10 {
		print!("{} ",cnt0);
		cnt0 += 1;
	}
	
	cnt0 = 0;
	print!("\nfor loop:");
	for i in 0..10 {
		print!("{i} ");
	}
	println!();
}
/*
fn plus_five(p1: i32) -> i32{
	p1 + 5	//equal "return p1+5"
}

fn guess_fun() {
	const RNG_MAX_RANG: u32 = 1_0;
	const RNG_MIN_RANG: u32 = 0x0_1;

	let secert_number = rand::thread_rng().gen_range(RNG_MIN_RANG..RNG_MAX_RANG);
	println!("\nthe secret is: {secert_number}");

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

fn type_fun(){
	let num: isize= 0x55aa;	//hex
	let emoji = '😂';	//char UTF-8
	let t: bool = true;	//bool [true, false]
	let tup: (u32, f32, char, &str, bool) = (0xff, 2.71828, 'ㄅ', " 😡 ", true);
	let (t0, t1, t2, t3, t4) = tup;
	let 星期ㄐ: [&str; 7] = ["Mon.", "Tue.", "Wed.", "Thu.", "Fri.", "Sat.", "Sun."];
	let x0 = [0; 4];	//equal "let x0 = [0, 0, 0, 0];""

	println!("num+10isize{}", num+10isize);
	println!("f64 PI: {PI}, MAX_10_EXP: {}", MAX_10_EXP);
	println!("u32 max: {MAX}, i32 min:{MIN}, bool: {t}, char: {}", emoji);
	println!("t0:{t0}, t1:{t1}, t2:{t2}, t3{t3}, t4:{t4}");
	println!("tup.0:{}, tup.4:{}星期ㄐ[0]:{}, x0[3]:{}", tup.0, tup.4, 星期ㄐ[0], x0[3]);
	println!();
}*/