use std::fmt::Debug;

pub trait Sorter: Debug {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Clone;
}

#[cfg(test)]
mod test {
    use super::*;
    #[derive(Debug)]
    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }
    #[test]
    fn std_works() {
        let mut things = vec![4, 2, 3, 1];
        StdSorter.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4]);
    }
}
