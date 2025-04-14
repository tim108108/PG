fn main() {
    println!("\nHello, world!");
    new_tuple();
    new_array();
    new_vector();
    new_struct();
    new_enum();
    new_if();
    new_match();
}

fn new_tuple() {
    let t_1 = (1, 2.2, 'c', "edfg");
    let (inedx1, size1, code1, name1): (i32, f32, char, &str) = t_1;
    let (inedx2, size2, code2, name2) = t_1;
    let x: () = (); //unit

    print!("\n======== fn new_tuple ========\n");

    println!("t_1.0: {}, t_1.1: {}, t_1.2: {}, t_1.3: {}",t_1.0, t_1.1, t_1.2, t_1.3);
    println!("inedx1: {}, size1: {}, code1: {}, name1: {}",inedx1, size1, code1, name1);
    println!("inedx2: {}, size2: {}, code2: {}, name2: {}",inedx2, size2, code2, name2);
    println!("{:?}", x);
}

fn new_array() {
    let arr_i32 = [1, 2];
    let arr_usize = [4, 3, 2, 1];
    let mut arr_usize_copy = arr_usize;
    let mut arr_f16:[f32; 8] = [1., 2., 3., 4., 5., 6., 7., 8.];
    let arr_f16_slice;
    let arr_char:[char; 4] = ['a', 'b', 'c', 'd'];
    print!("\n======== fn new_array ========\n");

    println!("arr_i32(len: {}): {:#?}", arr_i32.len(), arr_i32);
    
    for element in &mut arr_usize_copy {
        *element *= 2;
    }
    println!("arr_usize[0]: {}", arr_usize[0]);
    println!("arr_usize_copy[0]: {}", arr_usize_copy[0]);

    for element in arr_f16.iter_mut() {
        *element = *element*(*element+1.0);
    }
    arr_f16_slice = &arr_f16[5..];
    println!("arr_f16_slice: {:?}", arr_f16_slice);
    println!("arr_f16: {:?}", arr_f16);

    for element in arr_char {
        print!("{}", element);
    }
    print!("\n");
}

fn new_vector() {
    let v_zero = vec![0;8];
    let vec_i32 = vec!(1, 2, 3, 4);
    let mut vec_i32_clone = vec_i32.clone();
    let vec_char = vec!['a', 'b', 'c', 'd'];
    let mut v_new = Vec::new();
    let mut v_cap = Vec::with_capacity(3);
    print!("\n======== fn new_vector ========\n");


    println!("v_zero: {:?}", v_zero);
    
    
    for element in &mut vec_i32_clone {
        *element *= 10;
    }
    println!("vec_i32[0]: {}", vec_i32[0]);
    println!("vec_i32_clone: {:?}", vec_i32_clone);
    println!("vec_i32_clone.push(5): {:?}", vec_i32_clone.push(5));

    println!("vec_char: {:#?}", vec_char);

    v_new.push(4);
    println!("v_new: {:?}", v_new);
    println!("v_new.get(1): {:?}", v_new.get(1));

    println!("len: {}, cap: {}", v_cap.len(), v_cap.capacity());
    v_cap.push(1);
    v_cap.reserve(10);
    println!("len: {}, cap: {}", v_cap.len(), v_cap.capacity());
    v_cap.reserve(5);
    println!("len: {}, cap: {}", v_cap.len(), v_cap.capacity());
    v_cap.shrink_to_fit();
    println!("len: {}, cap: {}", v_cap.len(), v_cap.capacity());
}

fn new_struct() {
    #[derive(Debug, Clone)]
    struct Color(i32, i32, i32);
    let black = Color(1, 2, 3);

    #[derive(Debug, Clone)]
    struct Person{
        name: String,
        age: u32,
        email: String, // 最後一個字段的逗號可省略，但建議保留
        skin: Color,
      }
      
    let name = String::from("junmajinlong");
    let email = String::from("junmajinlong@xx.com");
    
    let user1 = Person{
    name,
    email,
    age: 23,
    skin: black.clone(),
    };
    
    let mut user2 = Person{
    name: String::from("gaoxiaofang"),
    email: String::from("gaoxiaofang@yy.com"),
    ..user1.clone()
    };
    user2.age = user2.age + 10;
    
    let user3 = Person{
        name: String::from("gaoxiaolin"),
        ..user2.clone()
        };

    print!("\n======== fn new_struct ========\n");
    println!("black: {}, {}, {}", black.0, black.1, black.2); 
    println!("user1.name: {}, user1.age: {}, user1.email: {}, user1.skin: {:?}", user1.name ,user1.age, user1.email, user1.skin);
    println!("user2: {:?}", user2); 
    println!("user3: {:?}", user3); 
    
}

fn new_enum() {
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }
    #[derive(Debug)]
    #[allow(dead_code)]
    enum BoardGameTurn {
        Move { squares: i32 },
        Pass,
    }
    print!("\n======== fn new_enum ========\n");
    let x: Message = Message::Move { x: 3, y: 4 };
    let y: BoardGameTurn = BoardGameTurn::Move { squares: 1 };
    println!("x: {:?}", x); 
    println!("y: {:?}", y); 

    let m = Message::Write("Hello, world 1".to_string());
    fn foo(x: String) -> Message {
        Message::Write(x)
    }
    let x = foo("Hello, world 2".to_string());
    println!("m: {:?}", m); 
    println!("x: {:?}", x);     

    let v = vec!["Hello".to_string(), "World".to_string()];
    let v1: Vec<Message> = v.into_iter().map(Message::Write).collect();
    println!("v1: {:?}", v1); 
}

fn new_if() {
    let x = 5;
    let y = if x == 5 { 10 } else { 15 }; // y: i32

    print!("\n======== fn new_if ========\n");
    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }
    println!("y = {:?}", y);
}

fn new_match() {
    #[allow(dead_code)]
    enum Message {
        Quit,
        ChangeColor(i32, i32, i32),
        Move { x: i32, y: i32 },
        Write(String),
    }
    
    fn quit() { /* ... */ }
    fn change_color(mut r: i32, mut g: i32, mut b: i32) { 
        println!("change_color old: R={}, G={}, B={}", r, g, b);
        r /= 2;
        g += 2;
        b *= 2;
        println!("change_color new: R={}, G={}, B={}", r, g, b);
    }
    fn move_cursor(x: i32, y: i32) {
        println!("move_cursor: x={}, y={}",x, y);
    }
    fn process_message(msg: Message) {
        match msg {
            Message::Quit => quit(),
            Message::ChangeColor(r, g, b) => change_color(r, g, b),
            Message::Move { x, y } => move_cursor(x, y),
            Message::Write(s) => println!("{}", s),
        };
    }
    print!("\n======== fn new_match ========\n");
    let x = 5;
    let y: Message = Message::Write("Hello, world".to_string());
    let z: Message = Message::ChangeColor(6,7,8);

    let number = match x {
        0 | 1 => "one",
        2 => "two",
        3 => "three",
        _ => "something else",
    };
    println!("number: {}",number);
    process_message(y);
    process_message(z);
}