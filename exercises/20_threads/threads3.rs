// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    let tx1 = mpsc::Sender::clone(&tx);
    let handle1 = thread::spawn(move || {
        for val in q.first_half {
            println!("sending {:?}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for val in q.second_half {
            println!("sending {:?}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}


//Les deux threads dans la fonction send_tx se terminent avant que la réception ne soit complète, car il n’y a pas de mécanisme pour garder le thread principal en vie jusqu’à ce que tous les messages soient reçus.
//Pour résoudre ce problème, j'utilise join pour attendre que les threads se terminent.
//J’ai donc ajouté des variables handle1 et handle2 pour stocker les handles des threads.
