impl Solution {

    fn quicksort_custom<H: Ord, N:Ord>(arrMaster: &mut [H], arrSlave: &mut [N]) {
        Self::_quicksort_custom(arrMaster, arrSlave, 0, (arrMaster.len()-1) as isize);
    }

    fn _quicksort_custom<H: Ord, N:Ord>(arrMaster: &mut [H], arrSlave: &mut [N], left_idx: isize, right_idx: isize) {
        if left_idx <= right_idx {
            let partition_idx = Self::partition(arrMaster, arrSlave, 0, right_idx);
            Self::_quicksort_custom(arrMaster, arrSlave, left_idx, partition_idx-1);
            Self::_quicksort_custom(arrMaster, arrSlave, partition_idx+1, right_idx);
        }
    }

    fn partition<H: Ord, N:Ord>(arrMaster: &mut [H], arrSlave: &mut [N], left_idx: isize, right_idx: isize) -> isize {
        let pivot = right_idx;
        let mut i: isize = left_idx-1;

        for j in left_idx..right_idx {
            if arrMaster[j as usize] >= arrMaster[pivot as usize] {
                i = i+1;
                arrMaster.swap(i as usize, j as usize);
                arrSlave.swap(i as usize, j as usize);
            }
        }

        arrMaster.swap((i+1) as usize, pivot as usize);
        arrSlave.swap((i+1) as usize, pivot as usize);

        return i+1
    }

    pub fn sort_people(mut names: Vec<String>, mut heights: Vec<i32>) -> Vec<String> {
        Self::quicksort_custom(heights.as_mut(), names.as_mut());
        return names;
    }
}
