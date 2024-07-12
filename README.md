with spin

```
$ spin up                                     
Warning: You're using a pre-release version of Spin (2.7.0-pre0). This plugin might not be compatible (supported: >=2.0). Continuing anyway.
Logging component stdio to ".spin/logs/"
file content is Err(Os { code: 44, kind: NotFound, message: "No such file or directory" })

```

with wasmtime

```
$ wasmtime run --dir src target/wasm32-wasi/release/job.wasm 
dirs [("src", "src")]
inside get_stdin for wasmtime
file content is Ok("use std::fs;\n\n#[allow(warnings)]\nmod bindings;\n\nfn main() {\n    spin_executor::run(async move {\n        let contents = fs::read_to_string(\"/src/main.rs\");\n        println!(\"file content is {:?}\", contents);\n    });\n}\n")
$ 

```