#!/bin/bash

set -eo pipefail

# This script runs hfscanner against a set of repositories.
# Base dir is the root directory containing all repositories.
BASE_DIR="/mnt/bigboy/pinned-action-repos"
# From here we expect the folders structure to be:
# BASE_DIR/<owner1>/<repo1>/
# BASE_DIR/<owner1>/<repo2>/
# BASE_DIR/<owner2>/<repo1>/
# ...

# Writes the CVS files to this repository.
# Each CSV file is named as <owner>.<repo>.csv
RESULTS_DIR="results"

# Assumes hfscanner is in the directory where the script is run
HFSCANNER_COMMAND="./hfscanner"

mkdir -p "$RESULTS_DIR"
echo "Starting scan of repositories in: $BASE_DIR"

for OWNER_DIR in "$BASE_DIR"/*/; do
    OWNER=$(basename "$OWNER_DIR")

    echo "Processing repositories for owner: $OWNER"

    # Iterate through each repository directory within the owner's directory
    for REPO_DIR in "$OWNER_DIR"*/; do
        REPO=$(basename "$REPO_DIR")
        FULL_REPO_PATH="$REPO_DIR"
        OUTPUT_CSV="$RESULTS_DIR/${OWNER}.${REPO}.csv"

        echo "  Running hfscanner for ${OWNOWNER}/${REPO}..."
        echo "    Repository Path: $FULL_REPO_PATH"
        echo "    Output CSV: $OUTPUT_CSV"

        "$HFSCANNER_COMMAND" "$FULL_REPO_PATH" --csv "$OUTPUT_CSV"
        EXIT_CODE=$?

        if [ $EXIT_CODE -eq 0 ]; then
            echo "  Successfully scanned ${OWNER}/${REPO}"
        else
            echo "  Error scanning ${OWNER}/${REPO}. Exit code: $EXIT_CODE"
        fi
        echo "----------------------------------------------------"
    done
done

echo "Script finished."
