#!/usr/bin/env python3
"""
Analysis script for hfscanner.csv data
Loads the CSV data into a pandas DataFrame and displays it with nice formatting
"""

import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
import os

# Set style for better looking plots
plt.style.use('default')
sns.set_palette("husl")

def load_hfscanner_data():
    """
    Load the hfscanner.csv data into a pandas DataFrame
    """
    # Get the current directory (results folder)
    current_dir = os.path.dirname(os.path.abspath(__file__))
    csv_file = os.path.join(current_dir, 'results/hfscanner.csv')

    # Load the CSV data
    df = pd.read_csv(csv_file)

    return df

def create_bubble_charts(df):
    """
    Create separate bubble charts for organizations and repositories
    """
    # Calculate total usages for each org and repo
    org_stats = df.groupby('org').agg({
        'safe_usages': 'sum',
        'partial_usages': 'sum',
        'unsafe_usages': 'sum',
        'file': 'count'
    }).reset_index()

    org_stats['total_usages'] = org_stats['safe_usages'] + org_stats['partial_usages'] + org_stats['unsafe_usages']

    # Exclude the huggingface org from the org bubble chart
    org_stats = org_stats[org_stats['org'] != 'huggingface']

    repo_stats = df.groupby(['org', 'repo']).agg({
        'safe_usages': 'sum',
        'partial_usages': 'sum',
        'unsafe_usages': 'sum',
        'file': 'count'
    }).reset_index()

    repo_stats['total_usages'] = repo_stats['safe_usages'] + repo_stats['partial_usages'] + repo_stats['unsafe_usages']

    # Create organizations bubble chart
    plt.figure(figsize=(14, 10))

    top_orgs = org_stats.nlargest(15, 'total_usages')

    # Scale bubble sizes to be more reasonable
    bubble_sizes_orgs = top_orgs['total_usages'] * 5  # Reduced from 25

    scatter_orgs = plt.scatter(
        top_orgs['total_usages'],
        top_orgs['file'],
        s=bubble_sizes_orgs,
        alpha=0.7,
        c=top_orgs['unsafe_usages'],  # Color based on unsafe usages
        cmap='Reds',
        edgecolors='black',
        linewidth=1
    )

    # Add labels for each bubble with better positioning
    for idx, row in top_orgs.iterrows():
        # Calculate label position to avoid overlap
        x_pos = row['total_usages']
        y_pos = row['file']

        # Adjust label position based on bubble size
        if row['total_usages'] > 2000:  # Large bubbles
            xytext = (10, 10)
            fontsize = 9
        elif row['total_usages'] > 500:  # Medium bubbles
            xytext = (8, 8)
            fontsize = 8
        else:  # Small bubbles
            xytext = (5, 5)
            fontsize = 7

        plt.annotate(
            row['org'],
            (x_pos, y_pos),
            xytext=xytext,
            textcoords='offset points',
            fontsize=fontsize,
            ha='left',
            va='bottom',
            weight='bold',
            bbox=dict(boxstyle='round,pad=0.3', facecolor='white', alpha=0.8, edgecolor='gray')
        )

    plt.xlabel('Total Hugging Face Usages', fontsize=12)
    plt.ylabel('Number of Files', fontsize=12)
    plt.title('Top Organizations by Hugging Face Usage\n(Bubble size = total usages, Color = unsafe usages)', fontsize=14, weight='bold')
    plt.grid(True, alpha=0.3)

    # Add colorbar for unsafe usages
    cbar_orgs = plt.colorbar(scatter_orgs)
    cbar_orgs.set_label('Unsafe Usages', fontsize=10)

    plt.tight_layout()

    # Save the organizations chart
    org_output_file = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'hfscanner_organizations.png')
    plt.savefig(org_output_file, dpi=300, bbox_inches='tight')
    print(f"Organizations chart saved to: {org_output_file}")

    plt.show()

    # Create repositories bubble chart
    plt.figure(figsize=(16, 12))

    top_repos = repo_stats.nlargest(20, 'total_usages')

    # Scale bubble sizes to be more reasonable
    bubble_sizes_repos = top_repos['total_usages'] * 3  # Reduced from 20

    scatter_repos = plt.scatter(
        top_repos['total_usages'],
        top_repos['file'],
        s=bubble_sizes_repos,
        alpha=0.7,
        c=top_repos['unsafe_usages'],  # Color based on unsafe usages
        cmap='Reds',
        edgecolors='black',
        linewidth=1
    )

    # Add labels for each bubble with better positioning
    for idx, row in top_repos.iterrows():
        # Calculate label position to avoid overlap
        x_pos = row['total_usages']
        y_pos = row['file']

        # Shorten repo names for better readability
        repo_label = row['repo'][:15] + '...' if len(row['repo']) > 15 else row['repo']
        full_label = f"{row['org']}/{repo_label}"

        # Adjust label position based on bubble size
        if row['total_usages'] > 1000:  # Large bubbles
            xytext = (12, 12)
            fontsize = 8
        elif row['total_usages'] > 300:  # Medium bubbles
            xytext = (8, 8)
            fontsize = 7
        else:  # Small bubbles
            xytext = (5, 5)
            fontsize = 6

        plt.annotate(
            full_label,
            (x_pos, y_pos),
            xytext=xytext,
            textcoords='offset points',
            fontsize=fontsize,
            ha='left',
            va='bottom',
            weight='bold',
            bbox=dict(boxstyle='round,pad=0.2', facecolor='white', alpha=0.8, edgecolor='gray')
        )

    plt.xlabel('Total Hugging Face Usages', fontsize=12)
    plt.ylabel('Number of Files', fontsize=12)
    plt.title('Top Repositories by Hugging Face Usage\n(Bubble size = total usages, Color = unsafe usages)', fontsize=14, weight='bold')
    plt.grid(True, alpha=0.3)

    # Add colorbar for unsafe usages
    cbar_repos = plt.colorbar(scatter_repos)
    cbar_repos.set_label('Unsafe Usages', fontsize=10)

    plt.tight_layout()

    # Save the repositories chart
    repo_output_file = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'hfscanner_repositories.png')
    plt.savefig(repo_output_file, dpi=300, bbox_inches='tight')
    print(f"Repositories chart saved to: {repo_output_file}")

    plt.show()

    return org_stats, repo_stats

def create_safety_trend_analysis(df):
    """
    Create safety trend analysis scatter plot showing safety ratio vs total usages
    """
    # Calculate safety metrics for each repository
    repo_safety = df.groupby(['org', 'repo']).agg({
        'safe_usages': 'sum',
        'partial_usages': 'sum',
        'unsafe_usages': 'sum',
        'file': 'count'
    }).reset_index()

    repo_safety['total_usages'] = repo_safety['safe_usages'] + repo_safety['partial_usages'] + repo_safety['unsafe_usages']

    # Calculate safety ratio (safe / total) - handle division by zero
    repo_safety['safety_ratio'] = repo_safety.apply(
        lambda row: row['safe_usages'] / row['total_usages'] if row['total_usages'] > 0 else 0,
        axis=1
    )

    # Filter out repos with very low usage (less than 1 total usages) for better analysis
    significant_repos = repo_safety[repo_safety['total_usages'] >= 1].copy()

    # Create the scatter plot
    plt.figure(figsize=(14, 10))

    # Create scatter plot with simple points
    plt.scatter(
        significant_repos['total_usages'],
        significant_repos['safety_ratio'],
        alpha=0.7,
        color='blue',
        edgecolors='black',
        linewidth=0.5
    )

    # Add labels for top repositories (high usage or high safety)
    top_repos = significant_repos.nlargest(15, 'total_usages')
    high_safety_repos = significant_repos[significant_repos['safety_ratio'] >= 0.1].nlargest(20, 'total_usages')  # Changed from 0.5 to 0.1 (10%)

    # Always include all repositories with safety ratio > 0.4
    very_safe_repos = significant_repos[significant_repos['safety_ratio'] > 0.4]

    # Combine and deduplicate for labeling
    labeled_repos = pd.concat([top_repos, high_safety_repos, very_safe_repos]).drop_duplicates(subset=['org', 'repo'])

    # Add labels for repositories meeting criteria: safety ratio > 0.4 OR total usages > 750
    labeled_repos = significant_repos[
        (significant_repos['safety_ratio'] > 0.4) |
        (significant_repos['total_usages'] > 750)
    ].copy()

    for idx, row in labeled_repos.iterrows():
        # Create shortened label
        repo_label = row['repo'][:12] + '...' if len(row['repo']) > 12 else row['repo']
        full_label = f"{row['org']}/{repo_label}"

        # Position label to avoid overlap
        x_pos = row['total_usages']
        y_pos = row['safety_ratio']

        # Adjust label position based on location
        if row['safety_ratio'] > 0.5:  # High safety
            xytext = (5, 5)
            fontsize = 8
        elif row['total_usages'] > 1000:  # High usage
            xytext = (5, -5)
            fontsize = 8
        else:
            xytext = (3, 3)
            fontsize = 7

        plt.annotate(
            full_label,
            (x_pos, y_pos),
            xytext=xytext,
            textcoords='offset points',
            fontsize=fontsize,
            ha='left',
            va='bottom',
            weight='bold',
            bbox=dict(boxstyle='round,pad=0.2', facecolor='white', alpha=0.8, edgecolor='gray')
        )

    # Add reference lines
    plt.axhline(y=0.5, color='green', linestyle='--', alpha=0.7, label='50% Safety Threshold')
    plt.axhline(y=0.25, color='orange', linestyle='--', alpha=0.7, label='25% Safety Threshold')
    plt.axhline(y=0.1, color='red', linestyle='--', alpha=0.7, label='10% Safety Threshold')

    # Add quadrants
    max_usage = significant_repos['total_usages'].max()
    plt.axvline(x=max_usage/2, color='gray', linestyle=':', alpha=0.5)

    # Add quadrant labels
    plt.text(max_usage*0.75, 0.75, 'High Usage\nHigh Safety', ha='center', va='center',
             bbox=dict(boxstyle='round,pad=0.5', facecolor='lightgreen', alpha=0.7))
    plt.text(max_usage*0.75, 0.25, 'High Usage\nLow Safety', ha='center', va='center',
             bbox=dict(boxstyle='round,pad=0.5', facecolor='lightcoral', alpha=0.7))
    plt.text(max_usage*0.25, 0.75, 'Low Usage\nHigh Safety', ha='center', va='center',
             bbox=dict(boxstyle='round,pad=0.5', facecolor='lightblue', alpha=0.7))
    plt.text(max_usage*0.25, 0.25, 'Low Usage\nLow Safety', ha='center', va='center',
             bbox=dict(boxstyle='round,pad=0.5', facecolor='lightyellow', alpha=0.7))

    plt.xlabel('Total Hugging Face Usages', fontsize=12)
    plt.ylabel('Safety Ratio (Safe/Total)', fontsize=12)
    plt.title('Safety Trend Analysis: Safety Ratio vs Total Usages', fontsize=14, weight='bold')

    plt.grid(True, alpha=0.3)
    plt.legend()

    plt.tight_layout()

    # Save the safety trend chart
    safety_output_file = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'hfscanner_safety_trend.png')
    plt.savefig(safety_output_file, dpi=300, bbox_inches='tight')
    print(f"Safety trend analysis saved to: {safety_output_file}")

    plt.show()

    # Print safety statistics
    print("\n" + "=" * 80)
    print("SAFETY TREND ANALYSIS STATISTICS")
    print("=" * 80)

    print(f"Total repositories analyzed: {len(significant_repos)}")
    print(f"Repositories with 50%+ safety ratio: {len(significant_repos[significant_repos['safety_ratio'] >= 0.5])}")
    print(f"Repositories with 25%+ safety ratio: {len(significant_repos[significant_repos['safety_ratio'] >= 0.25])}")
    print(f"Repositories with 10%+ safety ratio: {len(significant_repos[significant_repos['safety_ratio'] >= 0.1])}")
    print(f"Repositories with 0% safety ratio: {len(significant_repos[significant_repos['safety_ratio'] == 0])}")

    print("\nTop 10 Safest Repositories (50%+ safety ratio):")
    safest_repos = significant_repos[significant_repos['safety_ratio'] >= 0.5].nlargest(10, 'total_usages')
    for idx, row in safest_repos.iterrows():
        safety_pct = row['safety_ratio'] * 100
        print(f"  {row['org']}/{row['repo']}: {safety_pct:.1f}% safe ({row['total_usages']} total usages)")

    print("\nAll Repositories with 10%+ Safety Ratio:")
    good_safety_repos = significant_repos[significant_repos['safety_ratio'] >= 0.1].sort_values('safety_ratio', ascending=False)
    for idx, row in good_safety_repos.iterrows():
        safety_pct = row['safety_ratio'] * 100
        print(f"  {row['org']}/{row['repo']}: {safety_pct:.1f}% safe ({row['total_usages']} total usages)")

    print("\nHigh-Risk Repositories (High usage, Low safety):")
    high_risk = significant_repos[
        (significant_repos['total_usages'] > significant_repos['total_usages'].median()) &
        (significant_repos['safety_ratio'] < 0.1)
    ].nlargest(10, 'total_usages')

    for idx, row in high_risk.iterrows():
        safety_pct = row['safety_ratio'] * 100
        print(f"  {row['org']}/{row['repo']}: {safety_pct:.1f}% safe ({row['total_usages']} total usages)")

    return significant_repos

def display_dataframe_info(df):
    """
    Display information about the DataFrame
    """
    print("=" * 80)
    print("HFSCANNER DATA ANALYSIS")
    print("=" * 80)

    print(f"\nDataset Shape: {df.shape[0]} rows × {df.shape[1]} columns")
    print(f"Columns: {list(df.columns)}")

    print("\n" + "=" * 80)
    print("DATA TYPES")
    print("=" * 80)
    print(df.dtypes)

    print("\n" + "=" * 80)
    print("FIRST 10 ROWS")
    print("=" * 80)
    print(df.head(10).to_string(index=False))

    print("\n" + "=" * 80)
    print("BASIC STATISTICS")
    print("=" * 80)
    print(df.describe())

    print("\n" + "=" * 80)
    print("UNIQUE VALUES")
    print("=" * 80)
    print(f"Unique organizations: {df['org'].nunique()}")
    print(f"Unique repositories: {df['repo'].nunique()}")
    print(f"Files with safe usages: {len(df[df['safe_usages'] > 0])}")
    print(f"Files with partial usages: {len(df[df['partial_usages'] > 0])}")
    print(f"Files with unsafe usages: {len(df[df['unsafe_usages'] > 0])}")

    print("\n" + "=" * 80)
    print("USAGE SUMMARY")
    print("=" * 80)
    print(f"Total safe usages: {df['safe_usages'].sum()}")
    print(f"Total partial usages: {df['partial_usages'].sum()}")
    print(f"Total unsafe usages: {df['unsafe_usages'].sum()}")
    print(f"Total files analyzed: {len(df)}")

def main():
    """
    Main function to load and display the data
    """
    try:
        # Load the data
        df = load_hfscanner_data()

        # Exclude huggingface/transformers from all visualizations
        df_filtered = df[~((df['org'] == 'huggingface') & (df['repo'] == 'transformers'))].copy()

        # Display information about the DataFrame
        display_dataframe_info(df_filtered)

        # Create bubble charts
        print("\n" + "=" * 80)
        print("GENERATING BUBBLE CHARTS")
        print("=" * 80)
        org_stats, repo_stats = create_bubble_charts(df_filtered)

        # Create safety trend analysis
        print("\n" + "=" * 80)
        print("GENERATING SAFETY TREND ANALYSIS")
        print("=" * 80)
        safety_data = create_safety_trend_analysis(df_filtered)

        # Display top organizations and repositories
        print("\n" + "=" * 80)
        print("TOP 10 ORGANIZATIONS BY TOTAL USAGES")
        print("=" * 80)
        top_orgs = org_stats.nlargest(10, 'total_usages')[['org', 'total_usages', 'safe_usages', 'partial_usages', 'unsafe_usages', 'file']]
        print(top_orgs.to_string(index=False))

        print("\n" + "=" * 80)
        print("TOP 10 REPOSITORIES BY TOTAL USAGES")
        print("=" * 80)
        top_repos = repo_stats.nlargest(10, 'total_usages')[['org', 'repo', 'total_usages', 'safe_usages', 'partial_usages', 'unsafe_usages', 'file']]
        print(top_repos.to_string(index=False))

        return df_filtered

    except FileNotFoundError:
        print("Error: hfscanner.csv file not found in the results directory")
        return None
    except Exception as e:
        print(f"Error loading data: {e}")
        return None

if __name__ == "__main__":
    df = main()
