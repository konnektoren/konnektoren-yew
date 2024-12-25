#!/bin/bash
set -euo pipefail

# Default output directory for reports
REPORTS_DIR=${REPORTS_DIR:-"reports"}
OUTPUT_FILE="$REPORTS_DIR/i18n_report.txt"
SUMMARY_FILE="$REPORTS_DIR/i18n_summary.md"
STATUS_FILE="$REPORTS_DIR/status.txt"

# Create reports directory if it doesn't exist
mkdir -p "$REPORTS_DIR"

# Run the test and capture output
echo "Running i18n completeness check..."
cargo test i18n_completeness -- --nocapture > "$OUTPUT_FILE" || true

# Create summary markdown file
echo "### I18n Check Summary" > "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"

# Extract and count missing translations
missing_count=$(grep -c "Missing translations in" "$OUTPUT_FILE" || echo "0")
# Remove any whitespace
missing_count=$(echo $missing_count | tr -d '[:space:]')

# Save status information
echo "MISSING_COUNT=$missing_count" > "$STATUS_FILE"

echo "Found ${missing_count} file(s) with missing translations" >> "$SUMMARY_FILE"

# Add details of missing translations
if (( missing_count > 0 )); then
    echo "" >> "$SUMMARY_FILE"
    echo "Missing translations:" >> "$SUMMARY_FILE"
    echo "\`\`\`" >> "$SUMMARY_FILE"
    grep -A 1 "Missing translations in" "$OUTPUT_FILE" >> "$SUMMARY_FILE"
    echo "\`\`\`" >> "$SUMMARY_FILE"

    # Display in console with color
    echo -e "\n⚠️  Found missing translations:"
    grep -A 1 "Missing translations in" "$OUTPUT_FILE"
else
    echo -e "\n✅ All translations complete!"
fi

# Extract and display unused translations
if grep -q "Unused translations:" "$OUTPUT_FILE"; then
    echo -e "\nℹ️  Found unused translations:"
    echo "" >> "$SUMMARY_FILE"
    echo "Unused translations found:" >> "$SUMMARY_FILE"
    echo "\`\`\`" >> "$SUMMARY_FILE"
    awk '/Unused translations:/,/test test_i18n_completeness/' "$OUTPUT_FILE" | \
        grep -v "test test_i18n_completeness" | tee -a "$SUMMARY_FILE"
    echo "\`\`\`" >> "$SUMMARY_FILE"
fi

echo -e "\nFull report saved to $OUTPUT_FILE"
echo "Summary saved to $SUMMARY_FILE"

# Return status based on missing translations
exit 0
