fn main() {
    //plus 1 for what? because foreach starts from 0
    let numbers = 1..21;

    let language = vec!["rust", "javascript", "java", "php", "kotlin"];

    for i in numbers {
        println!("Numbers says {}", i);
    }

    // Without index number
    for i in language.iter() {
        println!("The language list: {}", i)
    }

    //With index number 
    for (index, i) in language.iter().enumerate() {
        println!("The language list: index {}, name {}", index, i)
    }

}