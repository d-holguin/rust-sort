use sorter::sort_evaluator::SortEvaluator;
use sorter::sorter::Sorter;
use sorter::sorts::gnomesort::GnomeSort;
use std::cell::Cell;
use std::error::Error;
use std::rc::Rc;

fn main() {
    let sorter = GnomeSort;
    let sort_size = 10_000;
    let counter = Rc::new(Cell::new(0));
    let rand = &mut rand::thread_rng();
    let values = SortEvaluator::<usize>::generate_values(sort_size, &counter, rand);
    let result = test_algorithm(&sorter, &values, &counter);
    match result {
        Ok(metrics) => println!(
            "{sorter:?}\n\
             Total Values: {total_values}\n\
             Comparisons: {count}\n\
             Time: {time:.3} seconds",
            sorter = sorter,
            total_values = values.len(),
            count = metrics.count,
            time = metrics.time
        ),
        Err(e) => eprintln!("Error: {}", e),
    }
}
#[derive(Debug)]
struct SortMetrics {
    count: usize,
    time: f64,
}

fn bench<T: Ord + Clone, S: Sorter>(
    sorter: &S,
    values: &mut [SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(values);
    let took = time.elapsed();
    let count = counter.get();
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    (count, took.as_secs_f64())
}

fn test_algorithm<T: Ord + Clone, S: Sorter>(
    sorter: &S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> Result<SortMetrics, Box<dyn Error>> {
    let result = bench(sorter, &mut values.to_vec(), counter);
    Ok(SortMetrics {
        count: result.0,
        time: result.1,
    })
}
