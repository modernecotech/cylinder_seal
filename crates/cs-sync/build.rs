fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .ok_or("cannot locate workspace root")?
        .join("proto");

    let proto_file = proto_root.join("chain_sync.proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile(&[proto_file], &[proto_root])?;

    Ok(())
}
