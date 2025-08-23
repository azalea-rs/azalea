use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=shaders");

    let out_dir = env::var("OUT_DIR").unwrap();
    let shader_dir = Path::new("shaders");

    let compiler = shaderc::Compiler::new().expect("failed to init shaderc");

    for entry in fs::read_dir(shader_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let kind = match ext {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => continue, // skip other files
        };

        let source = fs::read_to_string(&path).expect("failed to read shader");
        let filename = path.file_name().unwrap().to_str().unwrap();

        let artifact = compiler
            .compile_into_spirv(&source, kind, filename, "main", None)
            .expect("failed to compile shader");

        let spv_path = Path::new(&out_dir).join(format!("{}.spv", filename));
        fs::write(&spv_path, artifact.as_binary_u8()).unwrap();

        // Export an env var with uppercase name (TRIANGLE_VERT, TRIANGLE_FRAG, etc.)
        let env_var = filename.replace('.', "_").to_uppercase();
        println!("cargo:rustc-env={}={}", env_var, spv_path.display());
    }
}
