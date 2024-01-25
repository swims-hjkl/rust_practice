pub fn main(input_numbers: &mut Vec<f32>) {
    let high = input_numbers.len() - 1;
    quick_sort(input_numbers, 0, high);
    println!("{:?}", input_numbers);
}

fn quick_sort(array: &mut [f32], low: usize, high: usize) {
    if low < high {
        let partition_index = partition(array);
        quick_sort(&mut array[0..partition_index], 0, partition_index - 1);
        let next_high = array.len() - 1;
        quick_sort(
            &mut array[partition_index + 1..],
            partition_index + 1,
            next_high,
        );
    }
}

fn partition(array: &mut [f32]) -> usize {
    let mid: usize = array.len() / 2;
    let mut i = 0;
    let mut j = array.len() - 1;
    let pivot = array.len() - 1;
    (array[mid], array[j]) = (array[j], array[mid]);
    while i < j {
        loop {
            if array[i] >= array[pivot] {
                break;
            }
            i = i + 1;
        }
        loop {
            if array[j] < array[pivot] {
                break;
            }
            j = j - 1;
        }
        if i < j {
            (array[i], array[j]) = (array[j], array[i]);
        }
    }
    (array[i], array[pivot]) = (array[pivot], array[i]);
    i
}

// 2, 6, 3, 5, 3, 8, 7, 1, 0
// 2, 0, 3, 5, 3, 8, 7, 1, 6
// 2, 0, 1, 5, 3, 8, 7, 3, 6
// 2, 0, 1, 3, 5, 8, 7, 3, 6
//
// 2, 6, 3, 5, 8, 7, 1, 0, 3
// 2, 0, 3, 5, 8, 7, 1, 6, 3
// 2, 0, 1, 5, 8, 7, 3, 6, 3
//
//
// 2, 0, 1   //   5, 8, 7, 3, 6
//
// 5, 8, 7, 3, 6
// 5, 6, 7, 3, 8
// 5, 6, 3, 7, 8
//
// 5, 8, 3, 6, 7
// 5, 6, 3, 8, 7
// 5, 6, 3, 7, 8
//
// 2, 0, 1
// 0, 2, 1
