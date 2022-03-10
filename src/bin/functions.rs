fn main() {

    let name:String = String::from("Renci");
    display_name(name);

    for_number(20);

    if is_even(30) {
        println!("is even");
    } else {
        println!("is not a event");
    }
}

fn display_name(name: String) {
    println!("Your name is: {}", name);
}

fn for_number(int: u32) {
    for i in 1..int {
        println!("for_number say: {}", i)
    }
}

fn is_even(int: u32) -> bool {
    return int % 2 == 0;
}