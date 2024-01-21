#[derive(Debug)]
enum DoingSomething {
    V1(String),
    V2(i32),
    V3(Option<i32>),
}

pub fn main() {
    let v1 = DoingSomething::V1(String::from("34"));
    let v2 = DoingSomething::V2(23);
    let v3_data = DoingSomething::V3(Some(45));
    let v3_none = DoingSomething::V3(None);
    chooser(&v1);
    chooser(&v2);
    chooser(&v3_data);
    chooser(&v3_none);
    choose_only_string_with_match(&v1);
    choose_only_string_with_match(&v2);
    choose_only_string_with_if_let(&v1);
    choose_only_string_with_if_let(&v2);
}

fn chooser(value: &DoingSomething) {
    match value {
        DoingSomething::V1(string_value) => println!("It was a string: {string_value}"),
        DoingSomething::V2(integer_value) => println!("It was an integer: {integer_value}"),
        DoingSomething::V3(option_value) => match option_value {
            Some(option_value) => println!("It was an optional with value: {option_value}"),
            None => println!("It was an optional with value")
        },
    }
}

fn choose_only_string_with_match(value :&DoingSomething) {
    match value {
    DoingSomething::V1(string_value) => println!("It was string: {string_value}"),
    _ => println!("other things")
    }
}

fn choose_only_string_with_if_let(value :&DoingSomething) {
    if let DoingSomething::V1(val) = value {
        println!("This was a string {val}");
    }
    else {
        println!("This was someting else so else?");
    }
}
