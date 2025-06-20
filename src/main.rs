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
    PartiallySafe,
    Unsafe,
}

const EXCLUDED_DIRS: &[&str] = &[
    ".git", "node_modules", "__pycache__", ".mypy_cache", ".venv", "venv", ".env"
];

fn is_commit_sha(s: &str) -> bool {
    let sha_re = Regex::new(r"^[a-f0-9]{40}$").unwrap();
    sha_re.is_match(s)
}

fn scan_code_for_usage(code: &str) -> (usize, usize, usize) {
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
    let mut partial_count = 0;
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
                    partial_count += 1;
                }
            } else {
                unsafe_count += 1;
            }
        }
    }

    (safe_count, partial_count, unsafe_count)
}

fn scan_file(path: &Path) -> (usize, usize, usize) {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return (0, 0, 0),
    };
    scan_code_for_usage(&content)
}

fn is_excluded(entry: &walkdir::DirEntry) -> bool {
    entry.file_type().is_dir()
        && EXCLUDED_DIRS
            .iter()
            .any(|&e| entry.file_name().to_string_lossy().contains(e))
}

/// Extract (org, repo) from a path like `root/org/repo/file.py`
fn get_org_repo(path: &Path, root: &Path) -> (String, String) {
    let rel_components = match path.strip_prefix(root) {
        Ok(rel) => rel.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect::<Vec<_>>(),
        Err(_) => return ("unknown".to_string(), "unknown".to_string()),
    };

    if rel_components.len() < 3 {
        return ("unknown".to_string(), "unknown".to_string());
    }

    (rel_components[0].clone(), rel_components[1].clone())
}

fn write_file_csv(
    output_path: &str,
    file_data: &[(String, String, String, usize, usize, usize)],
) -> std::io::Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "org,repo,file,safe_usages,partial_usages,unsafe_usages")?;
    for (org, repo, file_path, safe, partial, unsafe_) in file_data {
        writeln!(writer, "{},{},{},{},{},{}", org, repo, file_path, safe, partial, unsafe_)?;
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
    let total_partial = Arc::new(Mutex::new(0));
    let total_unsafe = Arc::new(Mutex::new(0));
    let project_statuses = Arc::new(Mutex::new(HashMap::<(String, String), Status>::new()));
    let file_rows = Arc::new(Mutex::new(Vec::<(String, String, String, usize, usize, usize)>::new()));

    file_paths.par_iter().for_each(|entry| {
        let path = entry.path();
        let (safe, partial, unsafe_) = scan_file(path);

        if safe == 0 && partial == 0 && unsafe_ == 0 {
            return;
        }

        let (org, repo) = get_org_repo(path, &root_dir);
        let file_rel = path.strip_prefix(&root_dir).unwrap_or(path).to_string_lossy().to_string();

        file_rows.lock().unwrap().push((org.clone(), repo.clone(), file_rel, safe, partial, unsafe_));

        *total_safe.lock().unwrap() += safe;
        *total_partial.lock().unwrap() += partial;
        *total_unsafe.lock().unwrap() += unsafe_;

        let mut statuses = project_statuses.lock().unwrap();
        let key = (org.clone(), repo.clone());
        let current = statuses.get(&key).cloned();

        let new_status = if unsafe_ > 0 {
            Status::Unsafe
        } else if partial > 0 {
            Status::PartiallySafe
        } else {
            Status::Safe
        };

        let final_status = match (current, new_status) {
            (Some(Status::Unsafe), _) => Status::Unsafe,
            (_, Status::Unsafe) => Status::Unsafe,
            (Some(Status::PartiallySafe), _) => Status::PartiallySafe,
            (_, Status::PartiallySafe) => Status::PartiallySafe,
            _ => Status::Safe,
        };

        statuses.insert(key, final_status);
    });

    let total_safe_usages = *total_safe.lock().unwrap();
    let total_partial_usages = *total_partial.lock().unwrap();
    let total_unsafe_usages = *total_unsafe.lock().unwrap();
    let project_statuses = project_statuses.lock().unwrap();

    let safe_projects = project_statuses.values().filter(|&&s| s == Status::Safe).count();
    let partial_projects = project_statuses.values().filter(|&&s| s == Status::PartiallySafe).count();
    let unsafe_projects = project_statuses.values().filter(|&&s| s == Status::Unsafe).count();

    println!("====== Scan Summary ======");
    println!("Safe usages (with commit SHA): {}", total_safe_usages);
    println!("Partially safe usages (with tag/branch): {}", total_partial_usages);
    println!("Unsafe usages (no revision): {}", total_unsafe_usages);
    println!("Safe projects: {}", safe_projects);
    println!("Partially safe projects: {}", partial_projects);
    println!("Unsafe projects: {}", unsafe_projects);

    if detailed {
        println!("\n====== Project Status ======");
        for ((org, repo), status) in project_statuses.iter() {
            let status_str = match status {
                Status::Safe => "safe",
                Status::PartiallySafe => "partially_safe",
                Status::Unsafe => "unsafe",
            };
            println!("{:<20}/{:<20} {}", org, repo, status_str);
        }
    }

    if let Some(csv_file) = csv_output {
        if let Err(e) = write_file_csv(csv_file, &file_rows.lock().unwrap()) {
            eprintln!("Failed to write CSV: {}", e);
        } else {
            println!("CSV written to: {}", csv_file);
        }
    }
}
