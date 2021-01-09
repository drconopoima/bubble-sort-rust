use std::fmt::Debug;

pub fn bubble_sort<T: PartialOrd + Debug>(list: &mut [T]) {
    println!("original: {:?}", list);
    for ext_idx in 0..list.len() {
        let mut sorted = true;
        let limit_precursor = ext_idx as f32 / 2f32;
        let right_limit = limit_precursor.ceil() as usize;
        let left_limit = limit_precursor.floor() as usize;
        if ext_idx % 2 == 0 {
            for int_idx in (0 + left_limit)..(list.len() - 1 - right_limit) {
                if list[int_idx] > list[int_idx + 1] {
                    sorted = false;
                    list.swap(int_idx, int_idx + 1);
                }
            }
            println!("forward: {:?}", list);
        } else {
            for int_idx in ((1 + left_limit)..list.len() - right_limit).rev() {
                if list[int_idx] < list[int_idx - 1] {
                    sorted = false;
                    list.swap(int_idx - 1, int_idx);
                }
            }
            println!("backward: {:?}", list);
        }
        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sort() {
        use super::bubble_sort;
        let mut vector = vec![1, 3, 55, 7, 5, 100, 6, 41, 0, 2, 4];
        bubble_sort(&mut vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7, 41, 55, 100])
    }
}
