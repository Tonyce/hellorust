/**
* test特色台
*/

use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let a = "a";
    println!("Hello, world! {}", a);
    let future = hello_world(); // Nothing is printed
    block_on(future); 
}
