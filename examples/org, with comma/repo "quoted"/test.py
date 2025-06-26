from transformers import AutoModel

# Test file with comma and quotes in path
model = AutoModel.from_pretrained("model")