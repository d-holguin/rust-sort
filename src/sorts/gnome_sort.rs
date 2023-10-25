use crate::sorter::Sorter;

#[derive(Debug)]
pub struct GnomeSort;

impl Sorter for GnomeSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut pos = 0;
        while pos < slice.len() {
            if pos == 0 || slice[pos] >= slice[pos - 1] {
                pos += 1;
            } else {
                slice.swap(pos, pos - 1);
                pos -= 1;
            }
        }
    }
}

#[test]
fn gnome_works() {
    let mut things = vec![5, 6, 3, 1];
    GnomeSort.sort(&mut things);
    assert_eq!(things, &[1, 3, 5, 6]);
}
