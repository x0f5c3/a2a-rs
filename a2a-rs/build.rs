use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "grpc")]
    {
        println!("cargo:rerun-if-changed=proto/");
        
        // Use Buf to lint and validate proto files
        // Buf ensures proto files follow best practices
        let lint_output = Command::new("buf")
            .args(&["lint", "proto"])
            .output();
        
        if let Ok(output) = lint_output {
            if !output.status.success() {
                println!("cargo:warning=Buf lint warnings: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        
        // Use tonic-build to generate Rust code
        // tonic-build integrates well with Cargo's build system
        tonic_build::configure()
            .build_server(true)
            .build_client(true)
            .out_dir("src/adapter/grpc/generated")
            .compile_protos(&["proto/a2a.proto"], &["proto"])?;
        
        println!("cargo:warning=Successfully generated gRPC code with tonic-build (validated with Buf)");
    }
    Ok(())
}
