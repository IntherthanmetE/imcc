use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn show_spinner() {
    let frames = ["+", "x", "*", "x"];
    for i in 0..15 {
        print!("\r[{}] Compiling...", frames[i % frames.len()]);
        let _ = io::stdout().flush();
        thread::sleep(Duration::from_millis(80));
    }
    print!("\r                \r");
    let _ = io::stdout().flush();
}

fn run_compiler_c(code: &str) {
    println!("[C Compiler Mode] Code length: {}", code.len());
}

fn run_compiler_cpp(code: &str) {
    println!("[C++ Compiler Mode] Code length: {}", code.len());
}

fn run_compiler_go(code: &str) {
    println!("[Go Compiler Mode] Code length: {}", code.len());
}

fn run_compiler_rust(code: &str) {
    println!("[Rust Compiler Mode] Code length: {}", code.len());
}

fn run_compiler_java(code: &str) {
    println!("[Java Compiler Mode] Code length: {}", code.len());
}

fn run_compiler_csharp(code: &str) {
    println!("[C# Compiler Mode] Code length: {}", code.len());
}

fn run_compiler_swift(code: &str) {
    println!("[Swift Compiler Mode] Code length: {}", code.len());
}

fn run_compiler_kotlin(code: &str) {
    println!("[Kotlin Compiler Mode] Code length: {}", code.len());
}

fn run_interpreter_python(code: &str) {
    println!("[Python Interpreter Mode] Code length: {}", code.len());
}

fn run_interpreter_javascript(code: &str) {
    println!("[JavaScript Interpreter Mode] Code length: {}", code.len());
}

fn run_interpreter_typescript(code: &str) {
    println!("[TypeScript Interpreter Mode] Code length: {}", code.len());
}

fn run_interpreter_lua(code: &str) {
    println!("[Lua Interpreter Mode] Code length: {}", code.len());
}

fn run_interpreter_ruby(code: &str) {
    println!("[Ruby Interpreter Mode] Code length: {}", code.len());
}

fn run_interpreter_php(code: &str) {
    println!("[PHP Interpreter Mode] Code length: {}", code.len());
}

fn run_imcc_native(code: &str) {
    println!("[IMCC Native Mode] Code length: {}", code.len());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Missing file argument.");
        eprintln!("Usage: imcc <filename>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let path = Path::new(filename);
    
    let ext = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: Could not read file '{}'", filename);
            std::process::exit(1);
        }
    };

    show_spinner();

    match ext.as_str() {
        "c" => run_compiler_c(&content),
        "cpp" | "cc" | "cxx" => run_compiler_cpp(&content),
        "go" => run_compiler_go(&content),
        "rs" => run_compiler_rust(&content),
        "java" => run_compiler_java(&content),
        "cs" => run_compiler_csharp(&content),
        "swift" => run_compiler_swift(&content),
        "kt" | "kts" => run_compiler_kotlin(&content),
        "py" | "pyw" => run_interpreter_python(&content),
        "js" | "mjs" => run_interpreter_javascript(&content),
        "ts" => run_interpreter_typescript(&content),
        "lua" => run_interpreter_lua(&content),
        "rb" => run_interpreter_ruby(&content),
        "php" => run_interpreter_php(&content),
        _ => run_imcc_native(&content),
    }
}

