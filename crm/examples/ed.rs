use anyhow::Result;
use crm::UserInfo;
use prost::{bytes::BytesMut, Message};

fn main() -> Result<()> {
    let user = UserInfo::new(36, "kindy", "kindywu@qq.com");
    println!("{user:?}");
    let mut buf = BytesMut::new();
    user.encode(&mut buf)?;
    let user = UserInfo::decode(buf)?;
    println!("{user:?}");
    Ok(())
}
