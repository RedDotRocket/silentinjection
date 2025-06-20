# Unsafe / Safe Hugging Face Transformers Audit Tool

This tool scans Python codebases for potentially unsafe usage of Hugging Face's `transformers`, `datasets`, or `huggingface_hub` libraries, for unpinned revisions or unsafe revision values.

It detects:

- Model, tokenizer, dataset, file, or snapshot loading **without a pinned `revision`**
- Use of non-immutable `revision` values such as `"main"`, `"dev"`, or `"v1.0"`
- Absence of authentication (`use_auth_token=True`) or local paths

Only **40-character commit SHA hashes** in `revision="..."` are considered safe.

---

## Project Detection

Each **top-level subdirectory** inside the directory you scan is treated as a separate "project",
with the format `org/repo` where:

- `org` is the top-level directory name
- `repo` is the second-level directory name

Example:

```
root-folder/
├── project-1
│   └── repo-1
│       ├── mixed_unsafe.py
│       └── partially_safe_only.py
├── project-2
│   └── repo-2
│       ├── partially_safe_only.py
│       └── unsafe_only.py
├── project-3
│   └── repo-3
│       └── unsafe_only.py
└── project-4
    └── repo-4
        ├── mixed_partial.py
        └── safe_only.py
```

The scanner will extract:
- `org` and `repo` from the directory structure
- Usage statistics per file
- Aggregated safety status per `(org, repo)` pair

---

## Usage

### Build the Scanner

```bash
make release
```

This compiles an optimized binary to `target/release/hfscanner`.

---

### Run a Scan

#### 🔹 Summary only

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

This will also generate `results.csv` like:

```csv
org,repo,file,safe_usages,partial_usages,unsafe_usages
project-1,repo-1,project-1/repo-1/mixed_unsafe.py,3,2,2
project-2,repo-2,project-2/repo-2/unsafe_only.py,0,0,7
project-4,repo-4,project-4/repo-4/safe_only.py,8,0,0
```

Why list the org and repo twice? 

The second is the path to the file, which means we can seperate the same filename
in different projects, e.g. `repo-1/_init_.py` and `repo-2/subfolder/__init__.py`.

---

### Clean Build Artifacts

```bash
make clean
```

---

### Help

```bash
make help
```
