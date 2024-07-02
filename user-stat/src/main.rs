use anyhow::Result;
use user_stat::{IdQueryBuilder, QueryRequestBuilder, UserBuilder};

fn main() -> Result<()> {
    let u = UserBuilder::default()
        .name("kindy")
        .email("kindywu@outlook.com")
        .build()?;
    println!("{u:?}");

    let id = IdQueryBuilder::default().ids(vec![1312]).build()?;
    let q = QueryRequestBuilder::default()
        .id(("viewed_but_not_started".to_string(), id))
        .build()?;
    println!("{q:?}");

    Ok(())
}
