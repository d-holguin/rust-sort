use crate::sorter::Sorter;

#[derive(Debug)]
pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut n = slice.len();
        while n > 0 {
            let mut new_n = 0;
            for i in 1..n {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    new_n = i;
                }
            }
            n = new_n;
        }
    }
}

#[test]
fn bubble_works() {
    let mut things = vec![4, 2, 3, 1];
    BubbleSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4]);
}
