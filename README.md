## Example output
```
InsertionSort { smart: true }
Total Values: 50000
Comparisons: 711,335
Time: 0.3041 seconds

InsertionSort { smart: false }
Total Values: 50000
Comparisons: 625,217,462
Time: 19.6038 seconds

GnomeSort
Total Values: 50000
Comparisons: 1,249,616,281
Time: 29.0675 seconds

BubbleSort
Total Values: 50000
Comparisons: 1,249,753,508
Time: 39.4107 seconds

```

## Usage
```rust
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
                count = format_number_with_commas(metrics.count),
                time = metrics.time
            );
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

```