use std::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if worker_count == 0 {
        panic!("There must be at least one worker.");
    }

    let mut workers = Vec::with_capacity(worker_count);
    let mut counts = HashMap::new();
    for i in 0..worker_count {
        let mut t_input = Vec::new();
        for s in input.iter().skip(i).step_by(worker_count) {
            t_input.push(s.to_string().to_lowercase());
        }
        workers.push(thread::spawn(move || {
            let mut t_counts = HashMap::new();
            for s in t_input {
                for c in s.chars().filter(|s| s.is_alphabetic()) {
                    t_counts.entry(c)
                        .and_modify(|v| *v += 1)
                        .or_insert(1 as usize);
                }
            }
            t_counts
        }));
    }
    for t in workers {
        for e in t.join().unwrap().into_iter() {
            counts.entry(e.0)
                .and_modify(|v| *v += e.1)
                .or_insert(e.1);
        }
    }
    counts
}