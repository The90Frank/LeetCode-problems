impl Solution {
    fn heapify<H: Ord>(arr: &mut [H], n: usize, i: usize) {
        let mut largest_idx = i;
        let left_idx = 2 * i + 1;
        let right_idx = 2 * i + 2;

        if (left_idx < n && arr[left_idx] > arr[largest_idx]) {
            largest_idx = left_idx;
        }

        if (right_idx < n && arr[right_idx] > arr[largest_idx]) {
            largest_idx = right_idx;
        }

        if (largest_idx != i) {
            arr.swap(i, largest_idx);
            Self::heapify(arr, n, largest_idx);
        }
    }

    fn heapsort<H: Ord>(arr: &mut [H]) {
        for i in (0..(arr.len()/2)).rev() {
            Self::heapify(arr, arr.len(), i);
        }

        for i in (0..arr.len()).rev() {
            arr.swap(0, i);
            Self::heapify(arr, i, 0);
        }
    }

    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        Self::heapsort(nums.as_mut());
        return nums;
    }
}
