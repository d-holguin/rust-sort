use rayon::scope;
use sorter::sort_evaluator::{SortEvaluator, SortMetrics};
use sorter::sorter::Sorter;
use sorter::sorts::{BubbleSort, GnomeSort, InsertionSort};
use std::cell::Cell;
use std::error::Error;
use std::sync::Arc;

fn main() {
    scope(|s| {
        s.spawn(|_| {
            bench_sorter(InsertionSort { smart: false });
        });
        s.spawn(|_| {
            bench_sorter(InsertionSort { smart: true });
        });
        s.spawn(|_| {
            bench_sorter(BubbleSort);
        });
        s.spawn(|_| {
            bench_sorter(GnomeSort);
        });
    });
}

fn bench_sorter(sorter: impl Sorter) {
    let sort_size = 50_000;
    let counter = Arc::new(Cell::new(0));
    let rand = &mut rand::thread_rng();
    let mut values = SortEvaluator::<usize>::generate_values(sort_size, &counter, rand);
    let result = SortEvaluator::bench(&sorter, &mut values, &counter);
    print_metrics(result);
}

fn print_metrics(metrics: Result<SortMetrics, Box<dyn Error>>) {
    match metrics {
        Ok(metrics) => {
            println!(
                "{sorter}\n\
                 Total Values: {total_values}\n\
                 Comparisons: {count}\n\
                 Time: {time:.4} seconds\n",
                sorter = metrics.sorter,
                total_values = metrics.total_values,
                count = metrics.count,
                time = metrics.time
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
