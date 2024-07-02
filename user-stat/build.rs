use std::fs;

use anyhow::Result;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let protos = &[
        "../protos/user-stat/message.proto",
        "../protos/user-stat/rpc.proto",
    ];
    let builder = tonic_build::configure();
    builder.out_dir("src/pb").compile(protos, &["../protos"])?;

    Ok(())
}
