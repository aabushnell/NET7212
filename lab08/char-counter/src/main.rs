use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::thread;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// The file to count characters from
    file: String,

    /// The maximum level of parallelism to achieve [default: number of cores]
    #[arg(short, long, hide_default_value = true,
        default_value_t = thread::available_parallelism().unwrap().into())]
    max: usize,

    /// The number of times to run each test
    #[arg(short, long, default_value_t = 100)]
    times: u32,

    /// Display statistics
    #[arg(short, long, value_name = "rank")]
    stats: Option<usize>,
}

fn load_file(name: &str) -> Result<Vec<String>, io::Error> {
    io::BufReader::new(File::open(name)?)
        .lines()
        .collect()
}

fn count_chars<S: AsRef<str>>(input: &[S]) -> HashMap<char, usize> {
    let mut count = HashMap::new();
    for s in input {
        for c in s.as_ref().chars() {
            *count.entry(c).or_default() += 1;
        }
    }
    count
}

fn count_chars_mutex<S: AsRef<str>>(input: &[S],
    freq: &Mutex<HashMap<char, usize>>) {
    let mut freq = freq.lock().unwrap();
    for s in input {
        for c in s.as_ref().chars() {
            *freq.entry(c).or_default() += 1;
        }
    }
}

fn count_chars_parallel<S: AsRef<str> + Sync>(input: &[S], n: usize) ->
    HashMap<char, usize> {
    let (tx, rx) = channel();
    // let (tx, rx) = sync_channel(0);
    // would lead to deadlock as recv cannot be called
    // before the end of the scope, which requires threads
    // to terminate but they are all blocked on send

    let chunk_size = input.len().div_ceil(n).max(1);
    thread::scope(|s| {
        for chunk in input.chunks(chunk_size) {
            let tx = tx.clone();
            s.spawn(move || {
                let freq = count_chars(&chunk);
                tx.send(freq).unwrap();
            });
        }
    });

    drop(tx);
    let mut freq = HashMap::default();
    while let Ok(fr) = rx.recv() {
        for (c, n) in fr {
            *freq.entry(c).or_default() += n;
        }
    }
    freq
}

fn count_chars_parallel_join<S: AsRef<str> + Sync>(input: &[S], n: usize) ->
    HashMap<char, usize> {
    let mut freq = HashMap::default();

    let chunk_size = input.len().div_ceil(n).max(1);
    let mut chunks = input.chunks(chunk_size);

    thread::scope(|s| {
        let handles: Vec<_> = (0..n).map(|_| {
            let chunk = chunks.next().unwrap();
            s.spawn(move || count_chars(&chunk))
        }).collect();
        handles.into_iter().for_each(|h| {
            let fr = h.join().unwrap();
            for (c, n) in fr {
                *freq.entry(c).or_default() += n;
            }
        });
    });

    freq
}

fn count_chars_parallel_mutex<S: AsRef<str> + Sync>(input: &[S], n: usize) ->
    HashMap<char, usize> {
    let freq = Arc::new(Mutex::new(HashMap::default()));

    let chunk_size = input.len().div_ceil(n).max(1);
    thread::scope(|s| {
        for chunk in input.chunks(chunk_size) {
            let freq = Arc::clone(&freq);
            s.spawn(move || count_chars_mutex(&chunk, &freq));
        }
    });

    Arc::try_unwrap(freq).unwrap().into_inner().unwrap()
}

fn benchmark<S: AsRef<str> + Sync>(input: &[S], n: usize, times: u32,
    f: fn(&[S], usize) -> HashMap<char, usize>) -> Duration {
    let start = Instant::now();
    for _ in 0..times {
        std::hint::black_box(f(input, n));
    }
    start.elapsed() / times
}

fn benchmark_all<S: AsRef<str> + Sync>(input: &[S], max: usize, times: u32) {
    println!("");
    println!("Execution Time: `channel` implementation...");
    for n in 1..=max {
        let mean_elapsed = benchmark(input, n, times, count_chars_parallel);
        println!("  [{n} thread(s)]: {mean_elapsed:?}");
    }

    println!("");
    println!("Execution Time: `join` implementation...");
    for n in 1..=max {
        let mean_elapsed = benchmark(input, n, times, count_chars_parallel_join);
        println!("  [{n} thread(s)]: {mean_elapsed:?}");
    }

    println!("");
    println!("Execution Time: `Mutex` implementation...");
    for n in 1..=max {
        let mean_elapsed = benchmark(input, n, times, count_chars_parallel_mutex);
        println!("  [{n} thread(s)]: {mean_elapsed:?}");
    }
}

fn print_stats<S: AsRef<str> + Sync>(input: &[S], max: usize, rank: usize) {
    println!("");
    println!("Corpus Character Stats:");
    let freq = count_chars_parallel(input, max);
    let mut vec = freq.iter().collect::<Vec<(&char, &usize)>>();
    vec.sort_by_key(|&(_, v)| v);
    vec.into_iter()
        .rev()
        .take(rank)
        .for_each(
            |(k, v)| println!("- '{k}': {v} occurrences")
        );
}

fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let text = load_file(&args.file)?;
    let max = args.max;
    let times = args.times;

    println!("Running Benchmark w/ max {max} cores, {times} iterations!");

    benchmark_all(&text, args.max, args.times);

    if let Some(rank) = args.stats {
        print_stats(&text, max, rank);
    }

    Ok(())
}
