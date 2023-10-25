use super::{BubbleSort, GnomeSort, InsertionSort, QuickSort, SelectionSort};

#[derive(Debug)]
pub enum SortingAlgorithm {
    BubbleSort(BubbleSort),
    GnomeSort(GnomeSort),
    InsertionSort(InsertionSort),
    QuickSort(QuickSort),
    SelectionSort(SelectionSort),
}

impl TryFrom<&str> for SortingAlgorithm {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "bubblesort" => Ok(Self::BubbleSort(BubbleSort)),
            "gnomesort" => Ok(Self::GnomeSort(GnomeSort)),
            "insertionsort" => Ok(Self::InsertionSort(InsertionSort { smart: false })),
            "quicksort" => Ok(Self::QuickSort(QuickSort)),
            "selectionsort" => Ok(Self::SelectionSort(SelectionSort)),
            _ => Err(String::from("Invalid value for SortingAlgorithm")),
        }
    }
}

