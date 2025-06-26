from transformers import AutoModel

# Test file in directory with comma
model = AutoModel.from_pretrained("bert-base-uncased")