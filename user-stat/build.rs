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
        .with_field_attributes(
            &[
                "User.viewed_but_not_starteds",
                "User.started_but_not_finisheds",
            ],
            &[r#"#[sqlx(default)]"#],
        )
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
        .with_field_attributes(
            &["User.email", "User.name", "RawQueryRequest.query"],
            &[r#"#[builder(setter(into))]"#],
        )
        .with_field_attributes(
            &["TimeQuery.before", "TimeQuery.after"],
            &[r#"#[builder(setter(into, strip_option))]"#],
        )
        .with_field_attributes(
            &["QueryRequest.timestamps"],
            &[r#"#[builder(setter(each(name="timestamp", into)))]"#],
        )
        .with_field_attributes(
            &["QueryRequest.ids"],
            &[r#"#[builder(setter(each(name="id", into)))]"#],
        )
        .compile(protos, &["../protos"])?;

    Ok(())
}
