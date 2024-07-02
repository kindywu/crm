use anyhow::Result;
use proto_builder_trait::tonic::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let protos = &[
        "../protos/user-stat/message.proto",
        "../protos/user-stat/rpc.proto",
    ];
    let builder = tonic_build::configure();
    builder
        .with_serde(
            &["User"],
            true,
            true,
            Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        )
        .with_sqlx_from_row(&["User"], None)
        .out_dir("src/pb")
        .with_derive_builder(
            &[
                "User",
                "QueryRequest",
                "RawQueryRequest",
                "TimeQuery",
                "IdQuery",
            ],
            None,
        )
        .compile(protos, &["../protos"])?;

    Ok(())
}
