pub fn main(input_numbers: &mut Vec<f32>) {
    for i in 1..input_numbers.len() {
        for j in (1..=i).rev() {
            if input_numbers[j] < input_numbers[j - 1] {
                (input_numbers[j], input_numbers[j - 1]) = (input_numbers[j - 1], input_numbers[j]);
            }
        }
    }
}
