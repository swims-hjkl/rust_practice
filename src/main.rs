use std::io;
mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod selection_sort;

fn main() {
    // enum_example::main();
    let mut input_numbers: Vec<f32> = get_array_input();
    println!("");
    let sort_type = get_sort_type_selection();
    println!("");
    match sort_type.as_str().trim() {
        "1" => bubble_sort::main(&mut input_numbers),
        "2" => selection_sort::main(&mut input_numbers),
        "3" => insertion_sort::main(&mut input_numbers),
        "4" => merge_sort::main(&mut input_numbers),
        "5" => quick_sort::main(&mut input_numbers),
        _ => panic!("Something went wrong"),
    };
    println!("");
    println!("sorted array: {:?}", input_numbers);
}

fn get_sort_type_selection() -> String {
    let mut sort_type = String::new();
    println!("Select the type of sort you want to perform:");
    println!("1. bubble sort");
    println!("2. selection sort");
    println!("3. insertion sort");
    println!("4. merge sort");
    println!("5. quick sort");
    io::stdin()
        .read_line(&mut sort_type)
        .expect("Something went wrong taking input");
    sort_type
}

fn get_array_input() -> Vec<f32> {
    println!("Enter the numbers spereated by ,");
    let mut input_numbers = String::new();
    match io::stdin().read_line(&mut input_numbers) {
        Ok(value) => value,
        Err(_) => panic!("Something went wrong in having the names"),
    };
    // shadowing
    let input_numbers: Vec<f32> = input_numbers
        .split(",")
        .into_iter()
        .filter_map(|elem| match elem.trim().parse::<f32>() {
            Ok(val) => Some(val),
            Err(_) => None,
        })
        .collect();
    input_numbers
}
