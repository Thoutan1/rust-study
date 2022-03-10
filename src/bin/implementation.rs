struct Rectangle {
    width: u32,
    height: u32
}

struct Person {
    name: String,
    age: u8
}

impl Rectangle {
    fn printthis(&self) {
        println!("Rectangle: {}x{}", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("Your name is {} and your age {}", self.name, self.age);

    }
}

fn main() {

    //Change this value whatever
    let rect = Rectangle { width: 125, height: 125 };
    rect.printthis();
    // Is a square like 125 x 125 or not square 125 x 130 returning boolean
    println!("Is a square?: {}", rect.is_square());


    //Implementing Traits
    let jhon = Person { name: String::from("jhon"), age: 22 };
    println!("{}", jhon.to_string());
}