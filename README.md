## Example output
```
QuickSort
Total Values: 25000
Comparisons: 548,877
Time: 0.0174 seconds

MergeSort
Total Values: 25000
Comparisons: 333,976
Time: 0.0478 seconds

InsertionSort { smart: true }
Total Values: 25000
Comparisons: 330,668
Time: 0.1075 seconds

InsertionSort { smart: false }
Total Values: 25000
Comparisons: 156,662,963
Time: 5.2156 seconds

GnomeSort
Total Values: 25000
Comparisons: 310,906,832
Time: 7.7037 seconds

BubbleSort
Total Values: 25000
Comparisons: 312,443,748
Time: 10.7045 seconds

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