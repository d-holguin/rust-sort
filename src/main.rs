use rand::prelude::*;
use rust_sorting_visualizer::Sorter;
use rust_sorting_visualizer::sort_evaluator::SortEvaluator;
use rust_sorting_visualizer::sorts::bubblesort::BubbleSort;
use rust_sorting_visualizer::sorts::insertionsort::InsertionSort;
use rust_sorting_visualizer::sorts::quicksort::QuickSort;
use rust_sorting_visualizer::sorts::selectionsort::SelectionSort;
use std::cell::Cell;
use std::rc::Rc;



fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    println!("algorithm n comparisons time");
    for &n in &[0, 1, 10, 100, 1000, 5000] {
        let values = generate_values::<usize>(n, &counter, &mut rand);

        // run each sorting algorithm once per size of n
        test_algorithm("bubble", BubbleSort, &values, &counter);
        test_algorithm("insertion-smart", InsertionSort { smart: true }, &values, &counter);
        test_algorithm("insertion-dumb", InsertionSort { smart: false }, &values, &counter);
        test_algorithm("selection", SelectionSort, &values, &counter);
        test_algorithm("quick", QuickSort, &values, &counter);
    }
}


fn bench<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    let mut values: Vec<_> = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = counter.get();
    // assert!(values.is_sorted());
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    (count, took.as_secs_f64())
}

fn generate_values<T: Ord + Clone>(
    n: usize,
    counter: &Rc<Cell<usize>>,
    rand: &mut ThreadRng,
) -> Vec<SortEvaluator<usize>> {
    let mut values = Vec::with_capacity(n);
    for _ in 0..n {
        values.push(SortEvaluator {
            t: rand.gen::<usize>(),
            cmps: Rc::clone(&counter),
        });
    }
    values
}

fn test_algorithm<T: Ord + Clone, S: Sorter>(
    name: &str,
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) {
    let took = bench(sorter, &values, &counter);
    print_results(name, values.len(), took.0, took.1);
}

fn print_results(name: &str, n: usize, comparisons: usize, time: f64) {
    println!("{} {} {} {}", name, n, comparisons, time);
}