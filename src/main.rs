pub(crate) mod ast;
pub(crate) mod cli;
pub(crate) mod generator;
pub(crate) mod render;

use cli::Args;
use clap::Parser;
use rand::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

const FILE_PREFIXES: &[&str] = &[
    "main", "app", "index", "server", "client", "utils", "helpers", "config",
    "auth", "database", "db", "models", "routes", "middleware", "handler",
    "controller", "service", "api", "core", "engine", "parser", "lexer",
    "renderer", "manager", "factory", "builder", "adapter", "bridge",
    "observer", "strategy", "command", "state", "proxy", "decorator",
    "facade", "singleton", "prototype", "iterator", "mediator", "visitor",
    "cache", "queue", "stack", "tree", "graph", "node", "schema", "types",
    "constants", "errors", "logger", "metrics", "monitor", "worker",
    "scheduler", "dispatcher", "emitter", "listener", "subscriber",
    "publisher", "producer", "consumer", "validator", "sanitizer",
    "formatter", "transformer", "converter", "encoder", "decoder",
    "serializer", "deserializer", "migrator", "seeder", "fixture",
    "test_utils", "mock", "stub", "setup", "init", "bootstrap", "loader",
    "plugin", "extension", "module", "component", "widget", "layout",
    "view", "template", "style", "theme", "context", "provider", "store",
    "reducer", "action", "effect", "hook", "signal", "stream", "pipeline",
    "filter", "sort", "search", "fetch", "sync", "async_utils", "crypto",
    "hash", "token", "session", "cookie", "storage", "fs_utils", "io",
    "net", "http", "tcp", "udp", "websocket", "grpc", "rpc", "protocol",
];

const FILE_SUFFIXES: &[&str] = &[
    "", "_old", "_new", "_v2", "_backup", "_temp", "_final", "_draft",
    "_helper", "_impl", "_base", "_core", "_internal", "_public",
    "_test", "_spec", "_bench", "_debug",
];

fn generate_filenames(count: usize, ext: &str) -> Vec<String> {
    let mut rng = rand::rng();
    let mut used = HashSet::new();
    let mut names = Vec::with_capacity(count);

    for _ in 0..count {
        let mut filename;
        loop {
            let prefix = FILE_PREFIXES.choose(&mut rng).unwrap();
            let suffix = FILE_SUFFIXES.choose(&mut rng).unwrap();
            filename = format!("{}{}.{}", prefix, suffix, ext);
            if used.insert(filename.clone()) {
                break;
            }
        }
        names.push(filename);
    }
    names
}

fn main() {
    let vars: Vec<&'static str> = include_str!("names.txt").lines().collect();
    let args = Args::parse();

    let output_dir = PathBuf::from(&args.folder);
    if let Err(err) = fs::create_dir_all(&output_dir) {
        eprintln!("failed to create folder '{}': {}", args.folder, err);
        std::process::exit(1);
    }

    let filenames = generate_filenames(args.count, &args.extension);

    for filename in &filenames {
        let mut state = generator::GeneratorState::new(5, vars.clone());
        let program = state.generate_program();

        let output = match render::render_source_file(&program, &args.extension) {
            Ok(source) => source,
            Err(err) => {
                eprintln!("error: {}", err);
                std::process::exit(1);
            }
        };

        let output_path = output_dir.join(filename);
        if let Err(err) = fs::write(&output_path, &output) {
            eprintln!("failed to write '{}': {}", output_path.display(), err);
            std::process::exit(1);
        }

        println!("{}", output_path.display());
    }

    println!("generated {} files in {}", filenames.len(), args.folder);
}
