use anyhow::Result;
use crm::User;
use prost::{bytes::BytesMut, Message};

fn main() -> Result<()> {
    let user = User::new(36, "kindy", "kindywu@qq.com");
    println!("{user:?}");
    let mut buf = BytesMut::new();
    user.encode(&mut buf)?;
    let user = User::decode(buf)?;
    println!("{user:?}");
    Ok(())
}
