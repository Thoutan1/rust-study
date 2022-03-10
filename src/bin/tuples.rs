struct colors {
    red: u8,
    blue: u8,
    yellow: u8
}


fn main() {
    let tuple = (10, 20, 30, 40, 50);

    println!("Tuples say: {}", tuple.1); // Out 20

    //individual values in a tuple
    let tuple1: (i32, f64, u8) = (-222, 5.1, 23);
    println!("Integer say: {:?}",tuple.0);
    println!("Float say: {:?}",tuple.1);
    println!("Unsigned integer say: {:?}",tuple.2);


    //make tuple as parameter to a function
    let h: (i32, bool, u8) = (-115, true, 20);
    println!("function says: {:?}", h);

    //Tuple struct
    let mut bg = colors { red: 255, blue: 15, yellow: 70 };

    bg.yellow = 45;

    println!("Colors say: {}, {}, {}", bg.red, bg.blue, bg.yellow);
}   

fn printthis(i:(i32, bool, u8)) {
    println!("{:?}", i);
}