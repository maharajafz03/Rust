
// // mod server;
// // mod latest;
// // mod window;


// // #[derive(Debug,Clone)]
// // enum Maga {
// //    right,
// //     left
// // }

// // fn main() {
// //     let s = Maga::left;
// //     let b = s.clone();
// //     println!("{:?}", s);
// //     println!("{:?}", b);
// //   // server::server();
// //    // latest::latest();
// //     window::window(); 
// // }

// use std::thread::sleep;
// use std::time::Duration;

// fn cook_pasta() {
//     println!("Started cooking pasta...");
//     sleep(Duration::from_secs(3)); // Blocking
//     println!("Pasta is ready!");
// }

// fn cook_pizza() {
//     println!("Started cooking pizza...");
//     sleep(Duration::from_secs(4)); // Blocking
//     println!("Pizza is ready!");
// }

// fn main() {
//     println!("Restaurant is open!");
    
//     cook_pasta();  // Must wait for pasta to finish before starting pizza
//     cook_pizza();  

//     println!("All orders are served!");
// }


// use tokio::time::{sleep, Duration};

// async fn cook_pasta() {
//     println!("Started cooking pasta...");
//     sleep(Duration::from_secs(4)).await;  // Simulating cooking time
//     println!("Pasta is ready!");
// }

// async fn cook_pizza() {
//     println!("Started cooking pizza...");
//     sleep(Duration::from_secs(4)).await;  // Simulating cooking time
//     println!("Pizza is ready!");
// }

// #[tokio::main]
// async fn main() {
//     println!("Restaurant is open!");

//     // Start both cooking tasks at the same time
//     let pasta_task = tokio::spawn(cook_pasta());
//     let pizza_task = tokio::spawn(cook_pizza());

//     // Wait for both dishes to be ready
//     pasta_task.await.unwrap();
//     pizza_task.await.unwrap();

//     println!("All orders are served!");
// }


use std::thread;
use std::time::Duration;

fn main() {
    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(6));
        println!("Thread 1 finished!");
    });

    let handle2 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(2));
        println!("Thread 2 finished!");
    });

    let handle3 = thread::spawn(|| {
        thread::sleep(Duration::from_secs(3));
        println!("Thread 3 finished!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    println!("All threads finished!");
}

    //  use std::thread;
//use std::time::Duration;


    // let handle = thread::spawn(|| {
    //     println!("Thread started...");
    //     thread::sleep(Duration::from_secs(5)); // Blocks for 5 seconds
    //     println!("Thread finished!");
    // });

    // handle.join().unwrap(); // Main thread waits for child thread to finish
    // println!("Main thread done!");


//}

