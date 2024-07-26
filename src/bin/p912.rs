mod solution {
    use std::iter;

    use super::heap_sort;

    pub fn main(nums: Vec<i32>) -> Vec<i32> {
        const OFFSET: usize = 50003;
        let mut freq = vec![0; 100007];
        for &num in &nums {
            freq[num as usize + OFFSET] += 1;
        }
        freq.iter()
            .enumerate()
            .flat_map(|(num, &freq)| iter::repeat((num - OFFSET) as _).take(freq as _))
            .collect()
    }

    pub fn main_heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
        // the problem requires a sorting algorithm with O(nlogn) time complexity
        // and least possible space complexity, which is O(1).
        // so, we cannot use these popular sorting algorithms:
        // - quick sort, for its worst case time complexity being O(n^2);
        // - merge sort, for its space complexity is not O(1).
        // therefore, we implement heap sort, which satisfies all the requirements.
        heap_sort::heap_sort(&mut nums);
        nums
    }
}

mod heap_sort {
    /// Heap-sort the given slice in ascending order.
    /// 
    /// The implementation is based on the *standard implementation* mentioned in
    /// [Heapsort - Wikipedia](https://en.wikipedia.org/wiki/Heapsort).
    pub fn heap_sort<T: Ord>(data: &mut [T]) {
        let mut begin = data.len() / 2;
        let mut end = data.len();
        while end > 1 {
            if begin > 0 {
                begin -= 1;
            } else {
                end -= 1;
                data.swap(0, end);
            }

            let mut parent = begin;
            while parent * 2 + 1 < end {
                let lchild = parent * 2 + 1;
                let rchild = lchild + 1;

                let bigger_child = if rchild < end && data[rchild] > data[lchild] {
                    rchild
                } else {
                    lchild
                };

                if data[bigger_child] > data[parent] {
                    data.swap(bigger_child, parent);
                    parent = bigger_child;
                } else {
                    break;
                }
            }
        }
    }
}

fn main() {
    assert_eq!(solution::main(
        vec![5, 2, 3, 1]),
        vec![1, 2, 3, 5]);
    assert_eq!(solution::main(
        vec![5, 1, 1, 2, 0, 0]),
        vec![0, 0, 1, 1, 2, 5]);
    assert_eq!(solution::main_heap_sort(
        vec![5, 2, 3, 1]),
        vec![1, 2, 3, 5]);
    assert_eq!(solution::main_heap_sort(
        vec![5, 1, 1, 2, 0, 0]),
        vec![0, 0, 1, 1, 2, 5]);
}