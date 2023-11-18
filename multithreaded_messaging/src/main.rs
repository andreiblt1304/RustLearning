use std::sync::{mpsc, Mutex, Arc};
use std::sync::mpsc::Sender;
use std::thread;

pub fn spawn_and_print_thread(tx: Sender<String>, values: Vec<String>) {
    for val in values {
        tx.send(val).unwrap_or_else(|_| {
            println!("Error at sending the first values!")
        });
    }
}

pub fn share_mutex_between_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1: mpsc::Sender<String> = tx.clone();
    let first_values = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("first"),
        String::from("thread"),
    ];
    let second_values = vec![
        String::from("HELLO"),
        String::from("FROM"),
        String::from("THE"),
        String::from("SECOND"),
        String::from("THREAD"),
    ];

    share_mutex_between_threads();
    // spawn_and_print_thread(tx, first_values);
    // thread::sleep(Duration::from_secs_f32(0.5));   
    // spawn_and_print_thread(tx1, second_values);

    // for received in rx {
    //     println!("{}", received);
    // }    
}

