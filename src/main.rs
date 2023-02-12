use rand::prelude::*;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn bubble_sort(vec: &mut Vec<i32>) {
    let n = vec.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if vec[j] > vec[j + 1] {
                let temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
            }

            let mut output = String::new();
            for (idx, &x) in vec.iter().enumerate() {
                if (idx == j || idx == j + 1) && i != n - 2 {
                    output.push_str(&format!("\x1b[31m{}\x1b[0m ", x));
                } else if idx >= n - i || i == n - 2 {
                    output.push_str(&format!("\x1b[32m{}\x1b[0m ", x));
                } else {
                    output.push_str(&format!("{} ", x));
                }
            }
            output = format!("\rBubble sort: {}", output);
            print!("{}", output);
            stdout().flush().unwrap();

            sleep(Duration::from_millis(1));
        }
    }
    println!();
}

fn partition(vec: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = vec[high];
    let mut i = low;
    for j in low..high {
        if vec[j] <= pivot {
            vec.swap(i, j);
            i += 1;
        }
    }
    vec.swap(i, high);
    i
}

fn quick_sort(vec: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(vec, low, high);
        quick_sort(vec, low, pivot_index - 1);
        quick_sort(vec, pivot_index + 1, high);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let range = 1..25;
    let mut vec: Vec<i32> = range.clone().collect();
    vec.shuffle(&mut rng);
    let mut vec_1 = vec.clone();
    let mut vec_2 = vec.clone();
    let start = Instant::now();
    bubble_sort(&mut vec_1);
    let elapsed = start.elapsed();
    println!("Finished in {:?}", elapsed);
    //
    // println!("Range of {} to {}", range.start, range.end - 1);
    // let start = Instant::now();
    // let vec_2_len = vec_2.len();
    // quick_sort(&mut vec_2, 0, vec_2_len - 1);
    // let elapsed = start.elapsed();
    // println!("Finished in {:?}", elapsed);
}
