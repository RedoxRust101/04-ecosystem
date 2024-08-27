use anyhow::Result;
use bytes::{BufMut, BytesMut};

fn main() -> Result<()> {
  let mut buf = BytesMut::with_capacity(1024);
  buf.put(&b"hello world"[..]);
  buf.put_i64(0xdeadbeef);

  // b"hello world\0\0\0\0\xde\xad\xbe\xef"
  // null byte(\0) fills 64 bit empty space
  println!("buf: {:?}", buf);

  let a = buf.split();

  // buf will being empty now
  println!("a  : {:?}", a);
  println!("buf: {:?}", buf);

  let mut b = a.freeze();
  // b.put_u8(0xff); inner data cannot be changed
  let c = b.split_to(12);
  println!("b  : {:?}", b);
  println!("c  : {:?}", c);

  Ok(())
}
