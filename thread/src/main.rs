// use std::sync::mpsc;
use std::thread;
// use std::time::Duration;
// use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
    /* 1. 스레드의 기본
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("새 스레드: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("벡터: {:?}", v);
    });
    for i in 1..5 {
        println!("주 스레드: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    */

    /* 2. 채널 관련 코드
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("자식 스레드가"),
            String::from("안녕하세요"),
            String::from("라고"),
            String::from("인사를 합니다"),
        ];

        for val in vals {
            // tx.send(val).unwrap();
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        // let val = String::from("안녕하세요");
        // tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("그리고"),
            String::from("더 많은"),
            String::from("메시지를"),
            String::from("보냅니다."),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let received = rx.recv().unwrap();
    for received in rx {
        println!("수신: {}", received);
    }
    */
    /* 3. Mutex */
    let counter = Arc::new(Mutex::new(0));
    // Rc<T> 타입으로는 동시성을 해결할 수 없어 Arc<T> 타입을 사용한다
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
    println!("결과: {}", *counter.lock().unwrap());
}
