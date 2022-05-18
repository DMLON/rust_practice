fn multiply_pairs(pair: (i32, i32)) -> i32 {
    pair.0 * pair.1
}

// The following struct is for the activity.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn first_steps(){
    let x = 5 * 5;
    println!("Hello, world!, x = {}", x);
    // x = 90; ERROR, no es mutable

    let mut y = 5 * 5;
    println!("y = {}", y);
    y = 90;
    println!("y = {}", y);
    println!("{a},{b},{c}",
             a = "Banana",
             b = "Milanesa",
             c = "Pasta");


    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // println!("1u32 - 2 = {}", 1u32 - 2); ERROR overflow


    // Tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // Tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    
    // But long Tuples cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    println!("{:}",multiply_pairs((2,3)));


    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}

