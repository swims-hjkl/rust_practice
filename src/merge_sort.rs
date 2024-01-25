pub fn main(array: &mut Vec<f32>) {
    merge_sort(array);
}

fn merge_sort(a: &mut Vec<f32>) {
    if a.len() > 1 {
        let mid: usize = a.len() / 2;
        let mut left: Vec<f32> = a[0..mid].to_vec();
        let mut right: Vec<f32> = a[mid..a.len()].to_vec();
        merge_sort(&mut left);
        merge_sort(&mut right);
        merge(a, &left, &right);
    }
}

fn merge(array: &mut Vec<f32>, a1: &Vec<f32>, a2: &Vec<f32>) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < a1.len() && j < a2.len() {
        if a1[i] < a2[j] {
            array[k] = a1[i];
            i = i + 1;
            k = k + 1;
        } else {
            array[k] = a2[j];
            j = j + 1;
            k = k + 1;
        }
    }
    while i < a1.len() {
        array[k] = a1[i];
        i = i + 1;
        k = k + 1;
    }
    while j < a2.len() {
        array[k] = a2[j];
        k = k + 1;
        j = j + 1;
    }
}
