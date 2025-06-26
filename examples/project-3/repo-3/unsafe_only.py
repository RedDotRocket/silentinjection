from datasets import load_dataset
from huggingface_hub import hf_hub_download, snapshot_download
from transformers import AutoModel, AutoTokenizer

# All patterns without revision - should be marked as UNSAFE

# Unsafe model loading without revision
unsafe_model = AutoModel.from_pretrained("model")

# Unsafe tokenizer loading without revision
unsafe_tokenizer = AutoTokenizer.from_pretrained("model")

# Unsafe dataset loading without revision
unsafe_dataset = load_dataset("imdb")

# Unsafe file download without revision
unsafe_file_path = hf_hub_download(
    repo_id="deepseek-ai/DeepSeek-R1",
    filename="config.json"
)

# Unsafe snapshot download without revision
snapshot_download(repo_id="meta-llama/Llama-3.1-8B-Instruct")

# Another unsafe model
another_unsafe_model = AutoModel.from_pretrained("org/model")

# Another unsafe tokenizer  
another_unsafe_tokenizer = AutoTokenizer.from_pretrained("microsoft/DialoGPT-medium")