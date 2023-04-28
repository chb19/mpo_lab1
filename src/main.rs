use std::vec::Vec;
use std::time::Instant;

use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    
    let num_threads = num_cpus::get();
    let n = rng.gen_range(1..100_000_000) * num_threads;
    
    let vec : Vec<i64> = (0..n).map(|_| rng.gen_range(1..1000)).collect();
    let chunk_size = vec.len() / num_threads;

    println!("n: {}, num_threads: {}", n, num_threads);    
    {
        let start_iterative = Instant::now();
        let mut sum = 0;
        for el in &vec {
            sum += el;
        }
        println!("single-threaded, sum: {}, consumed time ms: {}", sum, start_iterative.elapsed().as_millis());
    }

    {
        crossbeam::thread::scope(|s| {
            let start_paralel = Instant::now();
            let mut handlers = Vec::with_capacity(num_threads);
            // Spawn threads to calculate sum for each chunk
            for i in 0..num_threads {
                let chunk = &vec[i * chunk_size..(i + 1) * chunk_size];
                let handle : crossbeam::thread::ScopedJoinHandle<i64> = s.spawn(move |_| {
                    chunk.iter().sum()
                });
                handlers.push(handle);
            }
        
            let mut results = handlers.into_iter().map(|x| x.join().unwrap()).collect::<Vec<i64>>();
            while results.len() > 1 {
                let mut handlers = Vec::with_capacity(results.len() / 2);
                for i in 0..results.len() / 2 {
                    let a = results[i];
                    let b = results[i + results.len() / 2];
                    let handle = s.spawn(move |_| {
                        a + b
                    });
                    handlers.push(handle);
                }
                results =  handlers.into_iter().map(|x| x.join().unwrap()).collect::<Vec<i64>>();
            }
            
            let sum = results[0];
            assert!(results.len() == 1);
            println!("computed sum by parallel algorithm: {}, time consumed: {}", sum, start_paralel.elapsed().as_millis());
        }).unwrap();
    }

}
