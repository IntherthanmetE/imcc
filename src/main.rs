use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn run_status(mut cmd: Command, tool_label: &str) -> i32 {
    match cmd.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            eprintln!("Error: '{}' not found in PATH.", tool_label);
            eprintln!("Install it and make sure it's on your PATH, then try again.");
            127
        }
        Err(e) => {
            eprintln!("Error running '{}': {}", tool_label, e);
            1
        }
    }
}

fn temp_bin_path(source: &Path) -> PathBuf {
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("imcc_out");
    let mut name = format!("imcc_{}_{}", stem, std::process::id());
    if cfg!(target_os = "windows") {
        name.push_str(".exe");
    }
    let mut p = env::temp_dir();
    p.push(name);
    p
}

fn compile_and_run(
    compiler: &str,
    compiler_args: &[&str],
    source: &Path,
    prog_args: &[String],
    keep: bool,
) -> i32 {
    let bin_path = temp_bin_path(source);

    println!("→ Compiling with {}...", compiler);
    let mut compile_cmd = Command::new(compiler);
    compile_cmd
        .args(compiler_args)
        .arg(source)
        .arg("-o")
        .arg(&bin_path);
    let compile_code = run_status(compile_cmd, compiler);
    if compile_code != 0 {
        return compile_code;
    }

    println!("→ Running...");
    let mut run_cmd = Command::new(&bin_path);
    run_cmd.args(prog_args);
    let run_label = bin_path.to_string_lossy().into_owned();
    let run_code = run_status(run_cmd, &run_label);

    if !keep {
        let _ = fs::remove_file(&bin_path);
    }

    run_code
}

fn run_direct(tool: &str, tool_args: &[&str], source: &Path, prog_args: &[String]) -> i32 {
    println!("→ Running with {}...", tool);
    let mut cmd = Command::new(tool);
    cmd.args(tool_args).arg(source).args(prog_args);
    run_status(cmd, tool)
}

fn run_java(source: &Path, prog_args: &[String], keep: bool) -> i32 {
    let class_name = match source.file_stem().and_then(|s| s.to_str()) {
        Some(s) => s.to_string(),
        None => {
            eprintln!(
                "Error: could not determine class name from '{}'.",
                source.display()
            );
            return 1;
        }
    };

    let out_dir = env::temp_dir().join(format!("imcc_java_{}", std::process::id()));
    if let Err(e) = fs::create_dir_all(&out_dir) {
        eprintln!(
            "Error: could not create temp directory '{}': {}",
            out_dir.display(),
            e
        );
        return 1;
    }

    println!("→ Compiling with javac...");
    let mut compile_cmd = Command::new("javac");
    compile_cmd.arg("-d").arg(&out_dir).arg(source);
    let compile_code = run_status(compile_cmd, "javac");
    if compile_code != 0 {
        let _ = fs::remove_dir_all(&out_dir);
        return compile_code;
    }

    println!("→ Running...");
    let mut run_cmd = Command::new("java");
    run_cmd.arg("-cp").arg(&out_dir).arg(&class_name).args(prog_args);
    let run_code = run_status(run_cmd, "java");

    if !keep {
        let _ = fs::remove_dir_all(&out_dir);
    }

    run_code
}

fn run_csharp(source: &Path, prog_args: &[String], keep: bool) -> i32 {
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("imcc_out");
    let mut bin_path = env::temp_dir();
    bin_path.push(format!("imcc_{}_{}.exe", stem, std::process::id()));

    let compiler = if cfg!(target_os = "windows") {
        "csc"
    } else {
        "mcs"
    };

    println!("→ Compiling with {}...", compiler);
    let mut compile_cmd = Command::new(compiler);
    compile_cmd
        .arg(format!("-out:{}", bin_path.display()))
        .arg(source);
    let compile_code = run_status(compile_cmd, compiler);
    if compile_code != 0 {
        return compile_code;
    }

    println!("→ Running...");
    let run_code = if cfg!(target_os = "windows") {
        let mut run_cmd = Command::new(&bin_path);
        run_cmd.args(prog_args);
        let label = bin_path.to_string_lossy().into_owned();
        run_status(run_cmd, &label)
    } else {
        let mut run_cmd = Command::new("mono");
        run_cmd.arg(&bin_path).args(prog_args);
        run_status(run_cmd, "mono")
    };

    if !keep {
        let _ = fs::remove_file(&bin_path);
    }

    run_code
}

fn run_kotlin_script(source: &Path, prog_args: &[String]) -> i32 {
    println!("→ Running with kotlin...");
    let mut cmd = Command::new("kotlin");
    cmd.arg(source).args(prog_args);
    run_status(cmd, "kotlin")
}

fn run_kotlin_compiled(source: &Path, prog_args: &[String], keep: bool) -> i32 {
    let stem = source
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("imcc_out");
    let mut jar_path = env::temp_dir();
    jar_path.push(format!("imcc_{}_{}.jar", stem, std::process::id()));

    println!("→ Compiling with kotlinc...");
    let mut compile_cmd = Command::new("kotlinc");
    compile_cmd
        .arg(source)
        .arg("-include-runtime")
        .arg("-d")
        .arg(&jar_path);
    let compile_code = run_status(compile_cmd, "kotlinc");
    if compile_code != 0 {
        return compile_code;
    }

    println!("→ Running...");
    let mut run_cmd = Command::new("java");
    run_cmd.arg("-jar").arg(&jar_path).args(prog_args);
    let run_code = run_status(run_cmd, "java");

    if !keep {
        let _ = fs::remove_file(&jar_path);
    }

    run_code
}

fn python_bin() -> &'static str {
    if cfg!(target_os = "windows") {
        "python"
    } else {
        "python3"
    }
}

fn print_usage() {
    eprintln!("imcc — run a source file with the right compiler/interpreter, automatically.");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  imcc [--keep] <file> [program args...]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --keep, -k   Keep the compiled binary/jar instead of deleting it after running");
    eprintln!("  -h, --help   Show this message");
    eprintln!();
    eprintln!("Supported extensions:");
    eprintln!("  Compiled:    c cpp cc cxx go rs java cs swift kt kts");
    eprintln!("  Interpreted: py pyw js mjs ts lua rb php");
}

fn main() -> ExitCode {
    let raw_args: Vec<String> = env::args().skip(1).collect();

    if raw_args.is_empty() {
        print_usage();
        return ExitCode::from(1);
    }
    if raw_args[0] == "-h" || raw_args[0] == "--help" {
        print_usage();
        return ExitCode::from(0);
    }

    let mut keep = false;
    let mut positional: Vec<String> = Vec::new();
    for a in raw_args {
        match a.as_str() {
            "--keep" | "-k" => keep = true,
            _ => positional.push(a),
        }
    }

    if positional.is_empty() {
        print_usage();
        return ExitCode::from(1);
    }

    let filename = positional[0].clone();
    let prog_args = positional[1..].to_vec();
    let path = Path::new(&filename);

    if !path.is_file() {
        eprintln!("Error: '{}' does not exist or is not a file.", filename);
        return ExitCode::from(1);
    }

    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let code: i32 = match ext.as_str() {
        "c" => compile_and_run("gcc", &[], path, &prog_args, keep),
        "cpp" | "cc" | "cxx" => compile_and_run("g++", &[], path, &prog_args, keep),
        "go" => run_direct("go", &["run"], path, &prog_args),
        "rs" => compile_and_run("rustc", &[], path, &prog_args, keep),
        "java" => run_java(path, &prog_args, keep),
        "cs" => run_csharp(path, &prog_args, keep),
        "swift" => run_direct("swift", &[], path, &prog_args),
        "kt" => run_kotlin_compiled(path, &prog_args, keep),
        "kts" => run_kotlin_script(path, &prog_args),
        "py" | "pyw" => run_direct(python_bin(), &[], path, &prog_args),
        "js" | "mjs" => run_direct("node", &[], path, &prog_args),
        "ts" => run_direct("npx", &["tsx"], path, &prog_args),
        "lua" => run_direct("lua", &[], path, &prog_args),
        "rb" => run_direct("ruby", &[], path, &prog_args),
        "php" => run_direct("php", &[], path, &prog_args),
        "" => {
            eprintln!(
                "Error: '{}' has no file extension — can't tell how to run it.",
                filename
            );
            1
        }
        other => {
            eprintln!("Error: unsupported extension '.{}'.", other);
            eprintln!(
                "Supported: c cpp cc cxx go rs java cs swift kt kts py pyw js mjs ts lua rb php"
            );
            1
        }
    };

    ExitCode::from(code as u8)
}
