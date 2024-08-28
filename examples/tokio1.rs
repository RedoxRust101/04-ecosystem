use std::{thread, time::Duration};

use tokio::{fs, runtime::Builder, time::sleep};
fn main() {
  let handler = thread::spawn(|| {
    let rt = Builder::new_current_thread().enable_all().build().unwrap();

    rt.spawn(async {
      println!("Future 1.");
      let content = fs::read_to_string("Cargo.toml").await.unwrap();
      println!("Content length: {}", content.len());
    });

    rt.spawn(async {
      println!("Future 2.");
      let ret = expensive_blocking_function("Future 2".to_string());
      println!("Result: {}", ret);
    });

    rt.block_on(async {
      sleep(Duration::from_millis(1000)).await;
    });
  });
  handler.join().unwrap();
}

fn expensive_blocking_function(s: String) -> String {
  std::thread::sleep(Duration::from_millis(800));
  blake3::hash(s.as_bytes()).to_string()
}
