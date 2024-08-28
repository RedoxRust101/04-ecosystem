// tokio async tash send message to worker for expensive blocking task
use anyhow::Result;
use std::{
  thread::{self, JoinHandle},
  time::Duration,
};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
  let (tx, rx) = mpsc::channel(32);
  let handler: JoinHandle<()> = worker(rx);

  tokio::spawn(async move {
    let mut i = 0;
    loop {
      i += 1;
      println!("Send task: {i}");
      tx.send(format!("task {i}")).await?;
    }
    #[allow(unreachable_code)]
    Ok::<(), anyhow::Error>(())
  });
  handler.join().unwrap();
  Ok(())
}

fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
  thread::spawn(move || {
    while let Some(s) = rx.blocking_recv() {
      let s1 = s.clone();
      let ret = expensive_blocking_function(s);
      println!("result: {} from {}", ret, s1);
    }
  })
}

fn expensive_blocking_function(s: String) -> String {
  thread::sleep(Duration::from_secs(1));
  blake3::hash(s.as_bytes()).to_string()
}
