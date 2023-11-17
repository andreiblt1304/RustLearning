use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    thread::spawn(move || {
        let first_values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("first"),
            String::from("thread"),
        ];

        for val in first_values {
            tx1.send(val).unwrap_or_else(|_| {
                println!("Error at sending the first values!")
            });
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let second_values = vec![
            String::from("HELLO"),
            String::from("FROM"),
            String::from("THE"),
            String::from("SECOND"),
            String::from("THREAD"),
        ];

        for val in second_values {
            tx.send(val).unwrap_or_else(|_| {
                println!("Error at sending the second values!")
            });
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("{}", received);
    }    
}

