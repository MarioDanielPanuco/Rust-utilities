pub fn merge_sort<T: Ord>(arr: &mut[T]) {
    let len = arr.len();

    if len > 1 {
        println!("{}", len);
        let mid_point = len / 2;
        merge_sort(&mut arr[..mid_point]);
        merge_sort(&mut arr[mid_point..]);
        merge(arr, mid_point);
    }
}

fn _merge_sort<T: Ord>(arr: &mut [T], start: isize, end: isize) {

    todo!()
}

fn merge<T: Ord>(arr: &mut[T], m: usize) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::merge_sort;

    #[test]
    fn merge_test() {
        let mut integer_arr: Vec<i32> = vec![15, 32, 2, 6, 3];

        merge_sort(&mut integer_arr);

        assert_eq!(integer_arr,vec![2, 3, 6, 15, 32]);

        let mut string_arr: Vec<String> = vec![
            String::from("Bob"),
            String::from("Steve"),
            String::from("Harry"),
            String::from("Peter"),
            String::from("Gwen")
        ];

        merge_sort(&mut string_arr);

        assert_eq!(string_arr, vec![
            String::from("Bob"),
            String::from("Gwen"),
            String::from("Harry"),
            String::from("Peter"),
            String::from("Steve")
        ])
    }
}