use std::io;

fn main() {
    let mut input = String::new();

    println!("Hei say something lol: ");

    match io::stdin() .read_line(&mut input) {
        Ok(_) => {
            println!("Oke you say {}", input);
        },
        Err(e) => println!("Something error lol {}", e),
    }
}