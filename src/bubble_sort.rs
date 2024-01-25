pub fn main(input_numbers: &mut Vec<f32>) {
    for i in 0..input_numbers.len() {
        for j in 0..(input_numbers.len() - i - 1) {
            if input_numbers[j] > input_numbers[j + 1] {
                (input_numbers[j], input_numbers[j + 1]) = (input_numbers[j + 1], input_numbers[j])
            }
        }
    }
}
