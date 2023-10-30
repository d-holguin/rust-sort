use crate::sorter::Sorter;


#[derive(Debug)]
pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice);
    }
}
fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }
    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;

    while left <= right {
        if &rest[left] <= pivot {
            //already on correct side
            left += 1;
        } else if &rest[right] > pivot {
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            // move element to the right
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }
    //re-algn left and right to account for pivot at 0
    let left = left + 1;
    //place the pivot at the its final location
    slice.swap(0, left - 1);
    // split where right starts
    let (left, right) = slice.split_at_mut(left - 1); // the pivot
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..]);
    //merge them
}

#[test]
fn quick_sort_works() {
    let mut things = vec![4, 2, 3, 5, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}
