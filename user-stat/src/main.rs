use anyhow::Result;
use user_stat::User;

fn main() -> Result<()> {
    let u = User::default();
    println!("{u:?}");
    Ok(())
}
