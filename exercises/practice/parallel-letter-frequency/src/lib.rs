use std::collections::HashMap;
use std::sync::Arc;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    let mut threads = Vec::with_capacity(worker_count);
    let mut vec = Vec::new();

    let it = input.iter();
    it.for_each(|string| {
        vec.push(string.to_lowercase());
    });

    let job = Arc::new(vec);
    let mut final_map: HashMap<char, usize> = HashMap::new();

    for i in 0..worker_count {
        let my_job = job.clone();
        threads.push(std::thread::spawn(move || { thread_work(i,worker_count,my_job) }));
    }


    for th in threads {
        let ret_map = th.join().unwrap();
        for entry in ret_map {
            final_map.entry(entry.0)
                .and_modify(|v| { *v = *v + entry.1 })
                .or_insert(entry.1);
        }
    }

    final_map
}

pub fn thread_work(id: usize, thread_count: usize, my_job: Arc<Vec<String>> ) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut iter = my_job.iter().skip(id).step_by(thread_count);
    while let Some(string) = iter.next() {
        string.chars()
            .for_each(|c: char| {
                if c.is_alphabetic() {
                    map.entry(c)
                        .and_modify(|v| { *v = *v + 1; })
                        .or_insert(1);
                };
            });
    };
    map
}
