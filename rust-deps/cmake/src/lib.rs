use std::env;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Config {
    source_dir: PathBuf,
    bin_dir: PathBuf,
    target: Option<String>,
    env: Vec<(OsString, OsString)>,
    generator: Option<String>,
}

impl Config {
    pub fn new<P: AsRef<Path>>(bin_dir: P) -> Config {
        Config {
            source_dir: bin_dir.as_ref().to_path_buf(),
            bin_dir: bin_dir.as_ref().to_path_buf(),
            target: None,
            env: Vec::new(),
            generator: None,
        }
    }
    pub fn source_dir<P: AsRef<Path>>(&mut self, source_dir: P) -> &mut Config {
        self.source_dir = source_dir.as_ref().to_path_buf();
        self
    }
    pub fn target(&mut self, target: &str) -> &mut Config {
        self.target = Some(target.to_string());
        self
    }
    pub fn env<K, V>(&mut self, key: K, value: V) -> &mut Config
    where
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.env
            .push((key.as_ref().to_owned(), value.as_ref().to_owned()));
        self
    }
    pub fn generator(&mut self, generator: &str) -> &mut Config {
        self.generator = Some(generator.to_string());
        self
    }
    pub fn build(&mut self) {
        let executable = env::var("CMAKE").unwrap_or("cmake".to_owned());
        if !self.bin_dir.join("CMakeCache.txt").is_file() {
            let _ = std::fs::create_dir(&self.bin_dir);
            self.run_cmake_source(&executable);
        }
        self.run_cmake_build(&executable);
    }
    fn run_cmake_source(&self, executable: &str) {
        let mut cmd = Command::new(executable);

        cmd.arg(&self.source_dir).current_dir(&self.bin_dir);
        if let Some(generator) = &self.generator {
            cmd.arg("-G").arg(generator);
        }
        cmd.arg("-DCMAKE_C_FLAGS= -ffunction-sections -fdata-sections");

        let profile = match env::var("PROFILE").unwrap().as_str() {
            "debug" => "Debug",
            "release" => "Release",
            unknown => {
                eprintln!(
                    "Warning: unknown Rust profile={}; defaulting to release build.",
                    unknown
                );
                "Release"
            }
        };
        cmd.arg(format!("-DCMAKE_BUILD_TYPE={}", profile));

        for &(ref k, ref v) in &self.env {
            cmd.env(k, v);
        }

        run(&mut cmd);
    }
    fn run_cmake_build(&self, executable: &str) {
        let mut cmd = Command::new(executable);
        cmd.arg("--build").arg(&self.bin_dir);

        if let Some(target) = &self.target {
            cmd.arg("--target").arg(target);
        }
        run(&mut cmd);
    }
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    match cmd.status() {
        Ok(status) if status.success() => {}
        Ok(status) => panic!("command did not execute successfully, got: {}", status),
        Err(e) => panic!("failed to execute command: {}", e),
    }
}
