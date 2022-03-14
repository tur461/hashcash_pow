use std::env;
use rand::Rng;
use std::thread;
use std::time::Instant;
use sha1::{Sha1, Digest};
use bus::{Bus, BusReader};
use std::sync::mpsc::{self, Sender, Receiver};

#[derive(Debug)]
struct SharedData {
    start_ctr: u64,
    end_ctr: u64,
    iterations: u64,
    hash_op: String,
    final_msg: String,
    thread_id: usize,
    found: bool
}

#[allow(non_snake_case)]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cmd <# of hex Zeros> <# of Threads>");
        return;
    }

    // here set # of zeros needed!
    let ZEROES: u8 = args[1].parse::<u8>().unwrap();
    let THREADS: usize = args[2].parse::<usize>().unwrap();
    let msg = String::from("Welcome to hashcash POW Algorithm, demo by tur461");
    println!("This Program, written in rust, demostrates HashCash type POW Algorithm/Concept used by Bitcoin BC");
    println!("Finding a hash with {} prefixed hex zeros and\nCorresponding message with SEED MSG: {}", ZEROES, msg);
    perform_pow(msg, ZEROES, THREADS);
}

#[allow(non_snake_case)]
fn perform_pow(msg: String, zeros: u8, THREADS: usize) {
    let mut rng = rand::thread_rng();
    let ctr_base: u64 = rng.gen();
    let diff = u64::MAX - ctr_base;
    let dv: u64 = diff/(THREADS as u64);
    let (tx, rx): (Sender<SharedData>, Receiver<SharedData>) = mpsc::channel();
    
    // outer-loop for finding suitable ctr!
    let mut bus_ = Bus::<bool>::new(THREADS);
    let start = Instant::now();
    for i in 0..THREADS {
        let tx_n = tx.clone();
        let m = msg.clone();
        let bs = bus_.add_rx();
        thread::spawn(move || search_for_hash(ctr_base, dv, i as u64, zeros, m, tx_n, bs));
    }
    
    println!("Calculating...");
    let sdat: SharedData = rx.recv().unwrap();
    if sdat.found {
        // lets send terminating signal to other threads!
        bus_.broadcast(true);
        let duration = start.elapsed().as_secs_f64();
        println!(
            "\nCompleted!:\n\nBy Thread #:\t\t{}\n\nHash:\t\t\t{}\nMSG:\t\t\t{}\nStart ctr:\t\t{}\nEnd ctr:\t\t{}\nctr count:\t\t{}", 
            sdat.thread_id,
            sdat.hash_op,
            sdat.final_msg,
            sdat.start_ctr,
            sdat.end_ctr,
            sdat.iterations
        );
        println!(
            "# of threads:\t\t{}\n# of Zeros (hex):\t{}\n# of zeros (bin):\t{}", 
            THREADS,
            zeros,
            zeros*4
        );
        println!(
            "Time taken:\t\t{} sec\nSpeed:\t\t\t{} iter/sec", 
            duration, 
            (sdat.iterations as f64/duration).ceil()
        );
    }
}

fn search_for_hash(
    ctr_base: u64, 
    dv: u64, 
    _i: u64, 
    zeros: u8, 
    msg: String, 
    prod: mpsc::Sender<SharedData>, 
    mut brx: BusReader<bool>
) {
    // println!("Starting thread # {}", _i);
    let mut ctr = ctr_base + (dv * _i);
    let end = ctr_base + (dv * (_i+1)) - 1;
    let msg_ = msg.as_str();
    loop {
        let mut _msg = String::from(msg_);
        _msg.push_str(&" ");
        _msg.push_str(&ctr.to_string());
        let mut hasher = Sha1::new();
        hasher.update(_msg.as_bytes());
        let result = hasher.finalize().to_vec();
        
        let num_of_zeros: usize = zeros as usize;
        let mut i = 0;
        let mut j = 0;
        let mut got_it: bool = true;
        
        // inner-loop for checking # of zeros!
        loop {
            match brx.try_recv() {
                Ok(found) => {
                    if found {
                        // terminating this loop (controlled by other threads using BUS)
                        // println!("Someone found the hash!. terminating thread # {}", _i);
                        break;
                    }
                }
                Err(..) => (),
            }
            if i>0 && i%2 == 0 {
                j += 1;
            }
            
            if i%2 == 0 {
                got_it = ((result[j] & 0xF0) >> 4) == 0;
            } else {
                got_it = (result[j] & 0x0F) == 0;
            }
            // terminating condition of inner loop
            if !got_it || i == (num_of_zeros - 1) {
                break;
            }
            i += 1;
        } 

        // terminating condition of outer loop
        if got_it && i == num_of_zeros-1 {
            let mut tmp_s: String = "".to_owned();
            for r in result {
                tmp_s.push_str(&tox(r).as_str());
            }
            let st = ctr_base + (dv * _i);
            let sdat = SharedData{
                start_ctr: st,
                end_ctr: ctr,
                iterations: ctr-st,
                hash_op: tmp_s,
                final_msg: _msg,
                thread_id: _i as usize,
                found: true,
            };
            // println!("Found in thread # {}.", _i);
            match prod.send(sdat) {
                Ok(_)  => {},
                Err(_) => println!("Receiver has stopped listening, dropping thread number {}.", _i),
            }
            break;
        }
        if ctr == end {
            // println!("Thread # {} completed.", _i);
            break;
        }
        ctr += 1;
    }
}

fn tox(d: u8) -> String {
    if d < 16 {
        format!("0{:x}", d)
    } else {
        format!("{:x}", d)
    }
}