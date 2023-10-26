use sorter::sort_evaluator::SortEvaluator;
use sorter::sorts::gnome_sort::GnomeSort;
use std::cell::Cell;
use std::sync::Arc;

fn main() {
    let sorter = GnomeSort;
    let sort_size = 10_000;
    let counter = Arc::new(Cell::new(0));
    let rand = &mut rand::thread_rng();
    let mut values = SortEvaluator::<usize>::generate_values(sort_size, &counter, rand);
    let result = SortEvaluator::bench(&sorter, &mut values, &counter);
    match result {
        Ok(metrics) => {
            println!(
                "{sorter}\n\
                 Total Values: {total_values}\n\
                 Comparisons: {count}\n\
                 Time: {time:.4} seconds",
                sorter = metrics.sorter,
                total_values = metrics.total_values,
                count = metrics.count,
                time = metrics.time
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
