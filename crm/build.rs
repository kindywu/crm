use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let builder = tonic_build::configure();
    builder.out_dir("src/pb").compile(
        &[
            "../protos/crm/crm.proto",
            "../protos/crm/message.proto",
            "../protos/crm/rpc.proto",
        ],
        &["../protos"],
    )?;
    Ok(())
}
