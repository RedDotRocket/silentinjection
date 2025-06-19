# Unsafe / Safe Hugging Face Transformers Audit Tool

This tool scans Python codebases for potentially unsafe usage of Hugging Face's `transformers`, `datasets`, or `huggingface_hub` libraries.

It detects:

- Model, tokenizer, dataset, file, or snapshot loading **without a pinned `revision`**
- Use of non-immutable `revision` values such as `"main"`, `"dev"`, or `"v1.0"`
- Absence of authentication (`use_auth_token=True`) or local paths

Only **40-character commit SHA hashes** in `revision="..."` are considered safe.

## Project Detection

Each **top-level subdirectory** inside the directory you scan is treated as a separate "project".

Example:

```
/codebase/
├── project_a/
│   └── file1.py
├── project_b/
│   └── file2.py
```

The scanner will report:
- `project_a` and `project_b` as individual projects
- Usage statistics are grouped per project

## Usage

### Build the Scanner

```bash
make release
```

This compiles an optimized binary to `target/release/hf_scanner`.

### Run a Scan

#### Summary only

```bash
make run-summary DIR=/path/to/codebase
```

#### Summary + per-project safety status

```bash
make run-detailed DIR=/path/to/codebase
```

#### Summary + per-project status + CSV report

```bash
make run-csv DIR=/path/to/codebase
```

This will also generate `report.csv` with:

```csv
project,status
project_a,unsafe
project_b,safe
```

### Clean Build Artifacts

```bash
make clean
```

### Help

```bash
make help
```
