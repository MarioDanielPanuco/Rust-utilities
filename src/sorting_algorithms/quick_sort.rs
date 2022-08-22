pub fn quick_sort<T: Ord>(arr: &mut [T]) {
let len = arr.len();
    _quick_sort(arr, 0, (len - 1) as isize);
}

fn _quick_sort<T: Ord>(arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let pivot = partition(arr, low, high);
        _quick_sort(arr, low, pivot - 1);
        _quick_sort(arr, pivot + 1, high);
    }
}

fn partition<T: Ord>(arr: &mut[T], low: isize, high: isize) -> isize {
    let pivot = high;
    let mut moving_index = low;

    for j in low..high {
        if arr[j as usize] <= arr[pivot as usize] {
            arr.swap(moving_index as usize, j as usize);
            moving_index += 1;
        }
    }
    arr.swap(moving_index as usize, pivot as usize);
    moving_index
}

#[cfg(test)]
mod tests {
    use super::quick_sort;

    #[test]
    fn quick_test() {
        let mut integer_arr: Vec<i32> = vec![15, 32, 2, 6, 3];

        quick_sort(&mut integer_arr);

        assert_eq!(integer_arr,vec![2, 3, 6, 15, 32]);

        let mut string_arr: Vec<String> = vec![
            String::from("Bob"),
            String::from("Steve"),
            String::from("Harry"),
            String::from("Peter"),
            String::from("Gwen")
        ];

        quick_sort(&mut string_arr);

        assert_eq!(string_arr, vec![
            String::from("Bob"),
            String::from("Gwen"),
            String::from("Harry"),
            String::from("Peter"),
            String::from("Steve")
        ])
    }
}