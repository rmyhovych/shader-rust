use shaderc;
use std::env;
use std::fs;

pub struct SpirV {
    artifact: shaderc::CompilationArtifact,
}

impl SpirV {
    pub fn write_to_file(&self, filepath: &String) {
        println!("Writing Compiled shader Spirv to path [{}]", filepath);

        let spirv_text = self.artifact.as_binary_u8();
        fs::write(filepath, spirv_text).expect("Unsuccessful file write!");
    }
}

pub struct ShaderCompiler {
    compiler: shaderc::Compiler,
}

impl ShaderCompiler {
    pub fn new() -> ShaderCompiler {
        let compiler = shaderc::Compiler::new().unwrap();
        ShaderCompiler { compiler }
    }

    fn compile(&mut self, path: &str, shader_kind: shaderc::ShaderKind) -> SpirV {
        println!("Compiling shader with path [{}]", path);

        let file_content =
            fs::read_to_string(path).expect(&format!("Unsuccessful file reading [{}].", path));

        let artifact: shaderc::CompilationArtifact = self
            .compiler
            .compile_into_spirv(&file_content, shader_kind, "shader.glsl", "main", None)
            .unwrap();

        println!("Compiled shader with path [{}]", path);
        SpirV { artifact }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Missing the filepath argument");
    }

    let filepath = &args[1];

    let shader_kind = match filepath.split(".").collect::<Vec<&str>>().last() {
        Some(kind_name) => match kind_name {
            &"vert" => shaderc::ShaderKind::Vertex,
            &"frag" => shaderc::ShaderKind::Fragment,
            _ => panic!("Unexpected file [{}]"),
        },
        None => panic!("Unexpected file [{}]"),
    };

    let mut shader_compiler = ShaderCompiler::new();
    let result: SpirV = shader_compiler.compile(filepath, shader_kind);

    let mut final_path = filepath.clone();
    final_path.push_str(".spv");
    result.write_to_file(&final_path);
}
