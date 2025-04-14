use tokio::signal::unix::{signal, SignalKind};


#[tokio::main]
async fn main() {
    println!("hello! Starting a wait for Ctrl+C");
    let mut signal = signal(SignalKind::terminate()).expect("unable to get sigterm signal.");

    let _ = signal.recv().await;

    println!("goodbye!");
}
