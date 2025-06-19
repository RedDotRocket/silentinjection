use walkdir::WalkDir;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::io::{Write, BufWriter};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Status {
    Safe,
    Unsafe,
}

const EXCLUDED_DIRS: &[&str] = &[
    ".git", "node_modules", "__pycache__", ".mypy_cache", ".venv", "venv", ".env"
];

fn is_commit_sha(s: &str) -> bool {
    let sha_re = Regex::new(r"^[a-f0-9]{40}$").unwrap();
    sha_re.is_match(s)
}

fn scan_code_for_usage(code: &str) -> (usize, usize) {
    let use_auth_or_local_re = Regex::new(r#"use_auth_token\s*=\s*True|from_pretrained\(["'](\./|/)"#).unwrap();
    let revision_capture_re = Regex::new(r#"revision\s*=\s*["']([^"']+)["']"#).unwrap();

    let patterns = vec![
        Regex::new(r#"AutoModel\.from_pretrained\s*\((?s:.*?)\)"#).unwrap(),
        Regex::new(r#"AutoTokenizer\.from_pretrained\s*\((?s:.*?)\)"#).unwrap(),
        Regex::new(r#"load_dataset\s*\((?s:.*?)\)"#).unwrap(),
        Regex::new(r#"hf_hub_download\s*\((?s:.*?)\)"#).unwrap(),
        Regex::new(r#"snapshot_download\s*\((?s:.*?)\)"#).unwrap(),
    ];

    let mut safe_count = 0;
    let mut unsafe_count = 0;

    for pattern in &patterns {
        for caps in pattern.captures_iter(code) {
            let full_call = caps.get(0).map_or("", |m| m.as_str());

            if use_auth_or_local_re.is_match(full_call) {
                safe_count += 1;
                continue;
            }

            if let Some(rev_caps) = revision_capture_re.captures(full_call) {
                let val = &rev_caps[1];
                if is_commit_sha(val) {
                    safe_count += 1;
                } else {
                    unsafe_count += 1;
                }
            } else {
                unsafe_count += 1;
            }
        }
    }

    (safe_count, unsafe_count)
}

fn scan_file(path: &Path) -> (usize, usize) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return (0, 0),
    };
    scan_code_for_usage(&content)
}

fn is_excluded(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_dir()
        && EXCLUDED_DIRS
            .iter()
            .any(|&e| entry.file_name().to_string_lossy().contains(e))
}

fn get_project_root(path: &Path, root_dir: &Path) -> String {
    path.strip_prefix(root_dir)
        .ok()
        .and_then(|rel| rel.components().next())
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn write_csv(output_path: &str, project_data: &HashMap<String, Status>) -> std::io::Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "project,status")?;
    for (project, status) in project_data {
        let status_str = match status {
            Status::Safe => "safe",
            Status::Unsafe => "unsafe",
        };
        writeln!(writer, "{},{}", project, status_str)?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <root_dir> [--summary | --detailed] [--csv <file>]", args[0]);
        return;
    }

    let root_dir = PathBuf::from(&args[1]);
    let detailed = args.contains(&"--detailed".to_string());
    let csv_index = args.iter().position(|x| x == "--csv");
    let csv_output = csv_index.and_then(|i| args.get(i + 1));

    let file_paths: Vec<_> = WalkDir::new(&root_dir)
        .into_iter()
        .filter_entry(|e| !is_excluded(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension().map_or(false, |ext| ext == "py"))
        .collect();

    let total_safe = Arc::new(Mutex::new(0));
    let total_unsafe = Arc::new(Mutex::new(0));
    let project_statuses = Arc::new(Mutex::new(HashMap::<String, Status>::new()));

    file_paths.par_iter().for_each(|entry| {
        let path = entry.path();
        let (safe, unsafe_) = scan_file(path);

        if safe == 0 && unsafe_ == 0 {
            return;
        }

        let project = get_project_root(path, &root_dir);

        if unsafe_ > 0 {
            *total_unsafe.lock().unwrap() += unsafe_;
            project_statuses.lock().unwrap().insert(project, Status::Unsafe);
        } else {
            *total_safe.lock().unwrap() += safe;
            project_statuses
                .lock()
                .unwrap()
                .entry(project)
                .or_insert(Status::Safe);
        }
    });

    let total_safe_usages = *total_safe.lock().unwrap();
    let total_unsafe_usages = *total_unsafe.lock().unwrap();
    let project_statuses = project_statuses.lock().unwrap();

    let safe_projects = project_statuses.values().filter(|&&s| s == Status::Safe).count();
    let unsafe_projects = project_statuses.values().filter(|&&s| s == Status::Unsafe).count();

    println!("====== Scan Summary ======");
    println!("Safe usages: {}", total_safe_usages);
    println!("Unsafe usages: {}", total_unsafe_usages);
    println!("Safe projects: {}", safe_projects);
    println!("Unsafe projects: {}", unsafe_projects);

    if detailed {
        println!("\n====== Project Status ======");
        for (project, status) in project_statuses.iter() {
            let status_str = match status {
                Status::Safe => "safe",
                Status::Unsafe => "unsafe",
            };
            println!("{:<40} {}", project, status_str);
        }
    }

    if let Some(csv_file) = csv_output {
        if let Err(e) = write_csv(csv_file, &project_statuses) {
            eprintln!("Failed to write CSV: {}", e);
        } else {
            println!("CSV written to: {}", csv_file);
        }
    }
}
