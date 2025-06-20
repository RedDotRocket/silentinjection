import pandas as pd
import matplotlib.pyplot as plt

INPUT_CSV = "results.csv"
PIE_FILE = "model_safety_pie_chart.png"

df = pd.read_csv(INPUT_CSV)

def classify_file(row):
    if row["unsafe_usages"] > 0:
        return "unsafe"
    elif row["partial_usages"] > 0:
        return "partially_safe"
    elif row["safe_usages"] > 0:
        return "safe"
    else:
        return "unknown"

df["status"] = df.apply(classify_file, axis=1)


status_counts = df["status"].value_counts()

print("===== File Safety Classification =====")
print(status_counts)


colors = {"safe": "green", "partially_safe": "orange", "unsafe": "red", "unknown": "gray"}
status_counts.plot.pie(
    autopct="%1.1f%%",
    figsize=(6, 6),
    startangle=90,
    colors=[colors.get(k, "gray") for k in status_counts.index],
    title="Hugging Face Model Pinning Classification (per file)"
)

plt.ylabel("")
plt.tight_layout()
plt.savefig(PIE_FILE)
print(f"\nSaved pie chart to: {PIE_FILE}")
