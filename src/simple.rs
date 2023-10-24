fn test_algorithm<T: Ord + Clone, S: Sorter>(
    name: &str,
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) {
    let took = bench(sorter, &values, &counter);

    print_results(name, values.len(), took.0, took.1);
}

fn print_header() {
    println!(
        "{:<20} | {:<10} | {:<15} | {:<10}",
        "Algorithm", "n", "Comparisons", "Time"
    );
    println!("{}", "-".repeat(60));
}

fn print_results(name: &str, n: usize, comparisons: usize, time: f64) {
    println!(
        "{:<20} | {:<10} | {:<15} | {:<10.6}",
        name, n, comparisons, time
    );
}
