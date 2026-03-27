use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let generated_dir = PathBuf::from("src/generated");
    if generated_dir.exists() {
        return Ok(());
    }

    std::fs::create_dir_all(&generated_dir)?;

    let mut config = prost_build::Config::new();
    config.out_dir(&generated_dir);

    prost_reflect_build::Builder::new()
        .file_descriptor_set_path(generated_dir.join("file_descriptor_set.bin"))
        .descriptor_pool("crate::generated::traces::DESCRIPTOR_POOL")
        .compile_protos_with_config(config, &["protos/traces.proto"], &["protos/"])?;

    Ok(())
}
