use crate::sorter::Sorter;

#[derive(Debug, Clone)]
pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Clone,
    {
        let len = slice.len();
        // Base case: if the slice has 1 or 0 elements, it's already sorted
        if len <= 1 {
            return;
        }
        // Find the middle index to divide the slice into two halves
        let mid = len / 2;
        // Recursively sort the left half and right half of the slice
        self.sort(&mut slice[..mid]);
        self.sort(&mut slice[mid..]);

        // create vector for merged results and pointers for left and rgiht halves
        let mut merged = Vec::with_capacity(len);
        let mut left = 0;
        let mut right = mid;

        // Merge the two sorted halves into the merged vector
        while left < mid && right < len {
            if slice[left] <= slice[right] {
                merged.push(slice[left].clone());
                left += 1; // Move to the next element in the left half
            } else {
                merged.push(slice[right].clone());
                right += 1; // Move to the next element in the right half
            }
        }

        // If there are any remaining elements in the left half, add them to merged
        while left < mid {
            merged.push(slice[left].clone());
            left += 1;
        }

        // If there are any remaining elements in the right half, add them to merged
        while right < len {
            merged.push(slice[right].clone());
            right += 1;
        }

        // Copy the sorted merged vector back into the original slice
        slice.clone_from_slice(&merged);
    }
}

#[test]
fn merge_sort_works() {
    let mut things = vec![4, 2, 3, 1];
    MergeSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
