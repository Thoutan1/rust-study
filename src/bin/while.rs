fn main() {
    let mut int = 1;

    while int <= 100 {

        if int % 10 == 0 {
            println!("int is {}", int)
        }

        
        int += 1;
    }
}