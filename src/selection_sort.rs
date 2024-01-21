
pub fn main(input_numbers :&mut Vec<f32>) -> &mut Vec<f32> {
    for i in 0..input_numbers.len() {
        let mut min_index = i;
        for j in i..input_numbers.len() {
            if input_numbers[j] < input_numbers[min_index] {
                min_index = j;   
            }
        }
        (input_numbers[i], input_numbers[min_index]) = (input_numbers[min_index], input_numbers[i]);
    }
    input_numbers
}
