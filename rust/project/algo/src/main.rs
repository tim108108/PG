fn main() {
    println!("\nHello, world!");
    new_tuple();
    new_array();
    new_vector();
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
    print!("\n======== fn new_vector ========\n");


    println!("v_zero: {:?}", v_zero);
    
    for element in &mut vec_i32_clone {
        *element *= 10;
    }
    println!("vec_i32[0]: {}", vec_i32[0]);
    println!("vec_i32_clone: {:?}", vec_i32_clone);

    println!("vec_char: {:#?}", vec_char);

    v_new.push(4);
    println!("v_new: {:?}", v_new);
    println!("v_new.get(1): {:?}", v_new.get(1));
}