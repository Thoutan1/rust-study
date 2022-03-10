fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for i in numbers.iter() {
        println!("{}\n", i);
    }

    for i in 0..numbers.len() {
        println!("{}", numbers[i]);
    }

    //Data types 
    let num: [i32;5] = [1, 2, 3, 4, 5];

    for i in 0..num.len() {
        println!("{}", num[i])
    }

    // make duplicate types example:
    let num2 = [5; 100];

    for i in 0..num2.len() {
        println!("{}", num2[i])
    }

}