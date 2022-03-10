struct Person {
    name: String,
    age: u8
}

trait HasVoice {
    fn speak(&self);

    fn can_speak(&self) -> bool;
}

impl HasVoice for Person {
    fn speak(&self) {
        println!("Hello my name is: {}", self.name);
    }

    fn can_speak(&self) -> bool {
        if self.age > 0 {
            return true
        } return false;
    }
}

fn main() {
    //change with whatever
    let person = Person {
        name: String::from("jho"),
        age: 0
    };

    println!("You {} can speak?: {}", person.name, person.can_speak());
}
