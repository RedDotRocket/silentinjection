use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Status {
    Safe,          // Has commit SHA
    PartiallySafe, // Has revision (tag/branch) but not SHA
    Unsafe,        // No revision at all
}

const EXCLUDED_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "__pycache__",
    ".mypy_cache",
    ".venv",
    "venv",
    ".env",
];

fn is_commit_sha(s: &str) -> bool {
    let sha_re = Regex::new(r"^[a-f0-9]{40}$").unwrap();
    sha_re.is_match(s)
}

fn scan_code_for_usage(code: &str) -> (usize, usize, usize) {
    let use_auth_or_local_re =
        Regex::new(r#"use_auth_token\s*=\s*True|from_pretrained\(["'](\./|/)"#).unwrap();
    let revision_capture_re = Regex::new(r#"revision\s*=\s*["']([^"']+)["']"#).unwrap();

    let patterns = vec![
        Regex::new(r#"AutoModel\.from_pretrained\s*\((?s:.*?)\)"#).unwrap(),
        Regex::new(r#"AutoTokenizer\.from_pretrained\s*\((?s:.*?)\)"#).unwrap(),
        Regex::new(r#"load_dataset\s*\((?s:.*?)\)"#).unwrap(),
        Regex::new(r#"hf_hub_download\s*\((?s:.*?)\)"#).unwrap(),
        Regex::new(r#"snapshot_download\s*\((?s:.*?)\)"#).unwrap(),
    ];

    let mut safe_count = 0; // Has commit SHA
    let mut partial_count = 0; // Has revision (tag/branch) but not SHA
    let mut unsafe_count = 0; // No revision at all

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

fn get_project_root(path: &Path, root_dir: &Path) -> String {
    path.strip_prefix(root_dir)
        .ok()
        .and_then(|rel| rel.components().next())
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn format_csv_field(field: &str) -> String {
    // Quote the field if it contains commas, quotes, or newlines
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

fn write_csv(output_path: &str, project_data: &HashMap<String, Status>) -> std::io::Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "project,status")?;
    for (project, status) in project_data {
        let status_str = match status {
            Status::Safe => "safe",
            Status::PartiallySafe => "partially_safe",
            Status::Unsafe => "unsafe",
        };
        let quoted_project = format_csv_field(project);
        writeln!(writer, "{},{}", quoted_project, status_str)?;
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} <root_dir> [--summary | --detailed] [--csv <file>]",
            args[0]
        );
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
        .filter(|e| e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "py"))
        .collect();

    let total_safe = Arc::new(Mutex::new(0));
    let total_partial = Arc::new(Mutex::new(0));
    let total_unsafe = Arc::new(Mutex::new(0));
    let project_statuses = Arc::new(Mutex::new(HashMap::<String, Status>::new()));

    file_paths.par_iter().for_each(|entry| {
        let path = entry.path();
        let (safe, partial, unsafe_) = scan_file(path);

        if safe == 0 && partial == 0 && unsafe_ == 0 {
            return;
        }

        let project = get_project_root(path, &root_dir);
        let mut project_statuses = project_statuses.lock().unwrap();

        // Update totals
        *total_safe.lock().unwrap() += safe;
        *total_partial.lock().unwrap() += partial;
        *total_unsafe.lock().unwrap() += unsafe_;

        // Determine project status based on priority: Unsafe > PartiallySafe > Safe
        if unsafe_ > 0 {
            project_statuses.insert(project, Status::Unsafe);
        } else if partial > 0 {
            project_statuses
                .entry(project)
                .or_insert(Status::PartiallySafe);
        } else if safe > 0 {
            project_statuses.entry(project).or_insert(Status::Safe);
        }
    });

    let total_safe_usages = *total_safe.lock().unwrap();
    let total_partial_usages = *total_partial.lock().unwrap();
    let total_unsafe_usages = *total_unsafe.lock().unwrap();
    let project_statuses = project_statuses.lock().unwrap();

    let safe_projects = project_statuses
        .values()
        .filter(|&&s| s == Status::Safe)
        .count();
    let partial_projects = project_statuses
        .values()
        .filter(|&&s| s == Status::PartiallySafe)
        .count();
    let unsafe_projects = project_statuses
        .values()
        .filter(|&&s| s == Status::Unsafe)
        .count();

    println!("====== Scan Summary ======");
    println!("Safe usages (with commit SHA): {}", total_safe_usages);
    println!(
        "Partially safe usages (with tag/branch): {}",
        total_partial_usages
    );
    println!("Unsafe usages (no revision): {}", total_unsafe_usages);
    println!("Safe projects: {}", safe_projects);
    println!("Partially safe projects: {}", partial_projects);
    println!("Unsafe projects: {}", unsafe_projects);

    if detailed {
        println!("\n====== Project Status ======");
        for (project, status) in project_statuses.iter() {
            let status_str = match status {
                Status::Safe => "safe",
                Status::PartiallySafe => "partially_safe",
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::NamedTempFile;

    #[test]
    fn test_format_csv_field_normal() {
        assert_eq!(format_csv_field("normal_project"), "normal_project");
        assert_eq!(format_csv_field("project-name"), "project-name");
        assert_eq!(format_csv_field("project_123"), "project_123");
    }

    #[test]
    fn test_format_csv_field_with_comma() {
        assert_eq!(
            format_csv_field("project, with comma"),
            "\"project, with comma\""
        );
        assert_eq!(format_csv_field("a,b,c"), "\"a,b,c\"");
        assert_eq!(
            format_csv_field("comma, at, multiple, places"),
            "\"comma, at, multiple, places\""
        );
    }

    #[test]
    fn test_format_csv_field_with_quotes() {
        assert_eq!(
            format_csv_field("project \"quoted\""),
            "\"project \"\"quoted\"\"\""
        );
        assert_eq!(format_csv_field("\"start quote"), "\"\"\"start quote\"");
        assert_eq!(format_csv_field("end quote\""), "\"end quote\"\"\"");
        assert_eq!(
            format_csv_field("multiple \"quotes\" here"),
            "\"multiple \"\"quotes\"\" here\""
        );
    }

    #[test]
    fn test_format_csv_field_with_newline() {
        assert_eq!(
            format_csv_field("project\nwith\nnewlines"),
            "\"project\nwith\nnewlines\""
        );
        assert_eq!(format_csv_field("line1\nline2"), "\"line1\nline2\"");
    }

    #[test]
    fn test_format_csv_field_combined_special_chars() {
        assert_eq!(
            format_csv_field("project, \"with\" both"),
            "\"project, \"\"with\"\" both\""
        );
        assert_eq!(
            format_csv_field("all: comma, \"quote\", and\nnewline"),
            "\"all: comma, \"\"quote\"\", and\nnewline\""
        );
    }

    #[test]
    fn test_write_csv_basic() -> std::io::Result<()> {
        let mut project_data = HashMap::new();
        project_data.insert("project1".to_string(), Status::Safe);
        project_data.insert("project2".to_string(), Status::Unsafe);
        project_data.insert("project3".to_string(), Status::PartiallySafe);

        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_str().unwrap();

        write_csv(temp_path, &project_data)?;

        let mut contents = String::new();
        let mut file = File::open(temp_path)?;
        file.read_to_string(&mut contents)?;

        assert!(contents.contains("project,status"));
        assert!(contents.contains("project1,safe"));
        assert!(contents.contains("project2,unsafe"));
        assert!(contents.contains("project3,partially_safe"));

        Ok(())
    }

    #[test]
    fn test_write_csv_with_special_chars() -> std::io::Result<()> {
        let mut project_data = HashMap::new();
        project_data.insert("normal_project".to_string(), Status::Safe);
        project_data.insert("project, with comma".to_string(), Status::Unsafe);
        project_data.insert("project \"quoted\"".to_string(), Status::PartiallySafe);
        project_data.insert("project\nwith\nnewline".to_string(), Status::Safe);

        let temp_file = NamedTempFile::new()?;
        let temp_path = temp_file.path().to_str().unwrap();

        write_csv(temp_path, &project_data)?;

        let mut contents = String::new();
        let mut file = File::open(temp_path)?;
        file.read_to_string(&mut contents)?;

        // Check header
        assert!(contents.contains("project,status"));

        // Check each entry is properly formatted
        assert!(contents.contains("normal_project,safe"));
        assert!(contents.contains("\"project, with comma\",unsafe"));
        assert!(contents.contains("\"project \"\"quoted\"\"\",partially_safe"));
        assert!(contents.contains("\"project\nwith\nnewline\",safe"));

        Ok(())
    }

    #[test]
    fn test_is_commit_sha() {
        // Valid SHA
        assert!(is_commit_sha("5d0f2e8a7f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d"));
        assert!(is_commit_sha("abcdef1234567890abcdef1234567890abcdef12"));

        // Invalid SHA
        assert!(!is_commit_sha("main"));
        assert!(!is_commit_sha("v1.0"));
        assert!(!is_commit_sha("5d0f2e8a7f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5")); // 39 chars
        assert!(!is_commit_sha("5d0f2e8a7f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d1")); // 41 chars
        assert!(!is_commit_sha("5D0F2E8A7F1B2C3D4E5F6A7B8C9D0E1F2A3B4C5D")); // uppercase
        assert!(!is_commit_sha("5g0f2e8a7f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d")); // contains 'g' (not hex!)
    }

    #[test]
    fn test_scan_code_for_usage_basic() {
        let code = r#"
from transformers import AutoModel
model = AutoModel.from_pretrained("bert-base-uncased")
"#;
        let (safe, partial, unsafe_) = scan_code_for_usage(code);
        assert_eq!(safe, 0);
        assert_eq!(partial, 0);
        assert_eq!(unsafe_, 1);
    }

    #[test]
    fn test_scan_code_for_usage_with_sha() {
        let code = r#"
from transformers import AutoModel
model = AutoModel.from_pretrained("bert-base-uncased", revision="5d0f2e8a7f1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d")
"#;
        let (safe, partial, unsafe_) = scan_code_for_usage(code);
        assert_eq!(safe, 1);
        assert_eq!(partial, 0);
        assert_eq!(unsafe_, 0);
    }

    #[test]
    fn test_scan_code_for_version_tag() {
        let code = r#"
from transformers import AutoModel
model = AutoModel.from_pretrained("bert-base-uncased", revision="v1.0.0")
"#;
        let (safe, partial, unsafe_) = scan_code_for_usage(code);
        assert_eq!(safe, 0);
        assert_eq!(partial, 1); // Tags are now counted as partially safe
        assert_eq!(unsafe_, 0);
    }

    #[test]
    fn test_scan_code_for_usage_with_main_tag() {
        let code = r#"
from transformers import AutoModel
model = AutoModel.from_pretrained("bert-base-uncased", revision="main")
"#;
        let (safe, partial, unsafe_) = scan_code_for_usage(code);
        assert_eq!(safe, 0);
        assert_eq!(partial, 1); // Tags are now counted as partially safe
        assert_eq!(unsafe_, 0);
    }
}
