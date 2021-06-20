
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("Say hello"),
            String::from("from"),
            String::from("child thread."),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("and"),
            String::from("send"),
            String::from("more messages"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    for received in rx {
        println!("received: {}", received);
    }
    
    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("vector: {:?}", v);
    });

    handle2.join().unwrap();

    let handle = thread::spawn(|| {
        for i  in 1..10 {
            println!("new thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
