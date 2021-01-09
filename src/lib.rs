use std::fmt::Debug;

pub fn bubble_sort<T: PartialOrd + Debug>(list: &mut [T]) {
    println!("original: {:?}", list);
    let mut right_skip = 1;
    let mut left_skip = 1;
    let mut last_swap_idx;
    let length = list.len();
    for ext_idx in 0..length {
        let mut sorted = true;
        let half_ext_idx = ext_idx as f32 / 2f32;
        if half_ext_idx as usize != 0 {
            let ceil_right = half_ext_idx.ceil() as usize;
            if right_skip < ceil_right {
                right_skip = ceil_right;
            }
            let floor_left = ceil_right - 1;
            if left_skip < floor_left {
                left_skip = floor_left;
            }
        }
        last_swap_idx = 0;
        if ext_idx % 2 == 0 {
            // println!("right_skip before fwd: {:?}", right_skip);
            // println!("left_skip before fwd: {:?}", left_skip);
            for int_idx in left_skip..(length - right_skip) {
                if list[int_idx] > list[int_idx + 1] {
                    sorted = false;
                    list.swap(int_idx, int_idx + 1);
                    last_swap_idx = int_idx;
                }
            }
            if last_swap_idx != 0 {
                right_skip = length - last_swap_idx - 1;
            }
            // println!("right_skip after fwd: {:?}", right_skip);
            println!("forward: {:?}", list);
        } else {
            // println!("right_skip before bck: {:?}", right_skip);
            // println!("left_skip before bck: {:?}", left_skip);
            for int_idx in (left_skip..(length - right_skip)).rev() {
                if list[int_idx] < list[int_idx - 1] {
                    sorted = false;
                    list.swap(int_idx - 1, int_idx);
                    last_swap_idx = int_idx;
                }
            }
            if last_swap_idx != 0 {
                left_skip = last_swap_idx;
            }
            // println!("left_skip after bck: {:?}", left_skip);
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
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7, 41, 55, 100]);
    }
    #[test]
    fn mostly_rightmost_ordered() {
        use super::bubble_sort;
        let mut vector = vec![5, 4, 0, 3, 2, 1, 6, 7, 41, 55, 100];
        bubble_sort(&mut vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7, 41, 55, 100]);
    }
    #[test]
    fn mostly_leftmost_ordered() {
        use super::bubble_sort;
        let mut vector = vec![0, 1, 2, 3, 4, 7, 100, 41, 5, 6, 55];
        bubble_sort(&mut vector);
        assert_eq!(vector, vec![0, 1, 2, 3, 4, 5, 6, 7, 41, 55, 100])
    }
    #[test]
    fn random_sort() {
        use super::bubble_sort;
        use rand::{distributions::Uniform, Rng};
        let mut rng = rand::thread_rng();
        let range = Uniform::new(0, 20);
        let random_vector: Vec<usize> = (0..100).map(|_| rng.sample(&range)).collect();
        let mut bubble_sorted = random_vector.clone();
        let mut std_sorted = random_vector.clone();
        std_sorted.sort();
        // std_sorted.push(1000);
        bubble_sort(&mut bubble_sorted);
        assert_eq!(bubble_sorted, std_sorted)
    }
}
