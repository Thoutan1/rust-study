fn main() {
    // not replaceable
    let int = 15;

    // replaceable 
    let mut int2 = 0;


    //ERROR
    if int != 15 {
        int += 1;
    }

    //SUCCCES
    if int2 != 10 {
        int2 += 10;
    }

    println!("int say {}", int);
    println!("int2 say {}", int2)
}