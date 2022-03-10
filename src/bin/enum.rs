enum language {
    javascript,
    php,
    rust,
    java
}

fn main() {
    //import reference enum types
    let language_list:language = language::rust;

    match language_list {
        language::java => println!("Java language"),
        language::rust => println!("rust language"),
        language::php => println!("php language"),
        language::javascript => println!("javascript language"),
    }
}