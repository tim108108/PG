use std::io;
use std::f64::consts::PI; //float f64 f32
use std::f32::MAX_10_EXP;
use std::u32::MAX; // unsign int
use std::i32::MIN; //int 
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	let enable = 0;
	println!("Hello, world!\n");

	if enable == 1{
	type_fun();
	guess_fun();
	println!("2+5={}",plus_five(2));
	common_fun();
	string_fun();
	struct_fun();
	rectangle_fun();
	}
	enum_fun();
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
	fn call(&self){}
}

#[derive(Debug)] // to show all US state
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
			println!("Lucy penny!!");
			1
		}
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
			println!("This Quarter is: {:?}!", state);
            25
		}
    }
}

fn enum_fun(){
	let home = IpAddr::V4(192, 168, 0, 1);
	let loopback = IpAddr::V6(String::from("..1"));
	println!("{:?}", home);
	println!("{:?}", loopback);

	let q = Message::Quit;
	let m = Message::Move { x: 12, y: 24 };
	let w = Message::Write(String::from("Hello"));
	let c = Message::ChangeColor(0, 255, 255);
	q.call();
	m.call();
	w.call();
	c.call();
	// enum Option<T> {	//<T>generics
	// 	None,
	// 	Some(T),
	// }
	let some_number = Some(5); 
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
	//example
	let six = plus_one(Some(5));
    let none = plus_one(None);
    println!("six = {:?} , none = {:?}", six, none);

	//match
	let c = Coin::Quarter(UsState::Alabama);
	println!("{}", value_in_cents(c));

	//Catch-all and _
	let v = 0u8;
	print!("v is :");
	match v {
		1 => println!("one"),
		2 => println!("two"),
		3 => println!("three"),
		_ => println!("others"),
	}
	let v = Some(0u8);
	match v {
		Some(3) => println!("three"),
		_ => (),
	}
	if let Some(3) = v{		// if let only care one case
		println!("three");
	}
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(Debug)]
struct Rectangle{
	width: u32,
	length: u32,
}
impl Rectangle{		//Methods 1
	fn area(&self) -> u32{
	self.width * self.length
	}
	fn can_hold(&self, other: &Rectangle) -> bool{
		self.width > other.width && self.length > other.length
	}
}
impl Rectangle{		//Methods 2
	fn square(size: u32) -> Rectangle{
		Rectangle {
			width: size,
			length: size,
		}
	}
}
fn rectangle_fun(){
		//debug
		let rect1 = Rectangle{
			width: 20,
			length: 50,
		};
		println!("area is {}", rect1.area());
		println!("debug: {:?}", rect1);	//debug type1
		println!("debug: {:#?}", rect1);	//debug type2
		let rect2 = Rectangle{
			width: 10,
			length: 20,
		};
		let rect3 = Rectangle{
			width: 50,
			length: 20,
		};
		println!("rect2.area(): {}, rect3.area(): {}", rect2.area(), rect3.area());
		println!("rect1.can_hold(&rect2): {}", rect1.can_hold(&rect2));
		println!("rect1.can_hold(&rect3): {}", rect1.can_hold(&rect3));

		let square1 = Rectangle::square(20);
		println!("square1.area(): {}", square1.area());
}

struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}
fn struct_fun(){
	let mut user1 = User {
		email: String::from("abc@123.com"),
		username: String::from("niga"),
		active: true,
		sign_in_count: 90,
	};
	user1.sign_in_count += 10;
	println!("name: {}\nemail:{}\nactive: {}\ncount: {}\n",user1.username, user1.email, user1.active, user1.sign_in_count);
	
	//pass struct to function
	let user2 = add_user("cba@123.com".to_string(), "gaga".to_string());
	println!("name: {}\nemail:{}\nactive: {}\ncount: {}\n",user2.username, user2.email, user2.active, user2.sign_in_count);
	
	//struct update
	let user3 = User{
		active: false,
		sign_in_count: 1,
		..user2
	};
	println!("name: {}\nemail:{}\nactive: {}\ncount: {}\n",user3.username, user3.email, user3.active, user3.sign_in_count);

	//tuple struct
	struct Point(i32, i32, i32);
	let ori = Point(1, 2, 3);
	println!("p1[{}], p2[{}], p3[{}]\n",ori.0, ori.1, ori.2);
}
fn add_user(email: String, username: String) -> User{
		User {
			email: email,
			username: username,
			active: true,
			sign_in_count: 0,
		}
}

fn string_fun(){
	//init str variable
	let mut s = String::from("First!!");
	println!("{}",s);
	s.push_str(", and second!!");
	println!("{}",s);
	
	//move 
	let s1 = String::from("this str1");
	let s2 = s1;
	println!("move: {}", s2);
	
	//clone (for heap)
	let s1 = String::from("this str");
	let s2 = s1.clone();
	println!("clone: {}, {}",s1, s2);

	//copy (for stack)
	let s1 = 5;
	let s2 = s1;
	println!("copy: {}, {}", s1, s2);

	//owner and function
	let s1 = String::from("take ownership");
	take_ownership(s1.clone());	//should use clone value pass  to function
	let s2 = 5;
	make_copy(s2);
	println!("pass to function, heap: {}, stack: {}", s1, s2);

	//return and scope
	let s1 = give_ownership();
	let s2 = String::from("str2");
	let s3 = take_and_give_bake(s2); //	move s2 to function
	let (s4, len) = calculate_length(s1.clone());	// clone s1 to function
	println!("s1: {}, s3: {}, s4: {}, len: {}", s1, s3, s4, len);

	//refer and borrow
	let s1 = String::from("12345");
	let len = calculate_refer_length(&s1);
	println!("s1: {}, refer s1 len: {}", s1, len);
	let mut s1 = String::from("12345");
	let len = add_refer_length(&mut s1);
	println!("s1: {}, refer s1 len: {}", s1, len);
	let mut s1 = String::from("string");
	{
		{
			let s2 = &mut s1;
			print!("{{ scope1 s2: {} [", s2);
		}
		let s3 = &mut s1;
		println!("scope2 s3: {} ] }}", s3);
	}
	
	//slice
	let s1 = String::from("Hello world");
	let s2 = &s1[0..5];		//[..5] same
	let s3 = &s1[6..11];	//[6.. s1.len()] or [6..] same
	let s4 = &s4[..];		//[.. s1.len()] or [0..] same
	let s5 = "hi word";
	println!("s2: {}, s3: {}, s4: {}", s2, s3, s4);

	println!("first word: {}", first_word(&s1[..]));	//string type
	println!("first word: {}", first_word(s5));			//&str type
}
fn take_ownership(some_string: String){
	println!("{}", some_string);
}
fn make_copy(some_number: i32){
	println!("{}",some_number);
}
fn give_ownership()->String{
	let some_string = String::from("str1");
	some_string
}
fn take_and_give_bake(some_string: String) -> String{
	some_string
}
fn calculate_length(s: String) -> (String, usize){
	let length = s.len();
	(s, length)
}
fn calculate_refer_length(s: &String) -> usize{
	s.len()
}
fn add_refer_length(s: &mut String) -> usize{
	s.push_str("6789");
	s.len()
}
fn first_word(s: &str) -> &str{
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate(){
		if item == b' '{
			return &s[..i];
		}
	}
	&s[..]
}

fn common_fun(){
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
	print!("\nwhile loop: ");
	while  cnt0 < 10 {
		print!("{} ",cnt0);
		cnt0 += 1;
	}
	
	print!("\n  for loop: ");
	for i in (0..10).rev() {	// reverse
		print!("{i} ");
	}
	println!();
}

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
}