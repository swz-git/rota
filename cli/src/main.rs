use std::{
    env::args,
    error::Error,
    io::{stdin, BufRead},
    sync::Arc,
    thread::{self, available_parallelism},
    time::Instant,
};

use crossbeam::{channel::unbounded, queue::SegQueue};
use rota::rank;

fn main() -> Result<(), Box<dyn Error>> {
    let term = args()
        .into_iter()
        .skip(1)
        .collect::<Vec<String>>()
        .join(" ");

    let (sender, receiver) = unbounded::<String>();
    let processed: Arc<SegQueue<(String, usize)>> = Arc::new(SegQueue::new());

    let mut threads = Vec::with_capacity(available_parallelism()?.into());

    for _i in 0..threads.capacity() {
        let (processed, receiver, term) = (processed.clone(), receiver.clone(), term.clone());
        threads.push(thread::spawn(move || {
            while let Ok(to_process) = receiver.recv() {
                if let Some(rank) = rank(&term, &to_process) {
                    processed.push((to_process, rank));
                }
            }
        }));
    }

    let before_read = Instant::now();
    let mut lines = stdin().lock().lines();
    while let Some(Ok(line)) = lines.next() {
        sender.send(line)?;
    }
    let read_time_taken = Instant::now().duration_since(before_read);

    // close channel --> threads quit
    drop(sender);

    let before_wait = Instant::now();
    threads.into_iter().for_each(|h| h.join().unwrap());
    let wait_time_taken = Instant::now().duration_since(before_wait);

    let mut vec = Vec::new();
    while let Some(item) = processed.pop() {
        vec.push(item);
    }

    let before_sort = Instant::now();
    // TODO: is sorting on insertion faster? (since were already read limited 99% of the time)
    vec.sort_by_cached_key(|x| x.1);
    let sort_time_taken = Instant::now().duration_since(before_sort);

    for (line, _) in vec.iter().rev() {
        println!("{line}");
    }
    eprintln!("read took {read_time_taken:?}");
    eprintln!("threads took {wait_time_taken:?}");
    eprintln!("sort took {sort_time_taken:?}");

    Ok(())
}
