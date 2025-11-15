use crate::EvalSummary;

/// Render a Markdown report for dashboards or CLI consumers.
pub fn render_markdown(summary: &EvalSummary) -> String {
    let mut out = String::new();
    out.push_str("# Mapping Evaluation Report\n\n");
    out.push_str(&format!(
        "*Total cases:* {}\\n\n*Precision:* {:.4}\\n\\n*Recall:* {:.4}\\n\\n*Accuracy:* {:.4}\\n\\n*F1:* {:.4}\\n\n\
*AutoMapped precision:* {:.4} ({} of {})\\n\n",
        summary.total_cases,
        summary.precision,
        summary.recall,
        summary.accuracy,
        summary.f1,
        summary.auto_mapped_precision,
        summary.auto_mapped_correct,
        summary.auto_mapped_total
    ));

    out.push_str("## Calibration buckets\n\n");
    out.push_str(
        "_Only MappingResults with an NCIt prediction contribute to calibration buckets._\n\n",
    );
    out.push_str("| Bucket | Range | Predictions | Correct | Accuracy |\\n| --- | --- | ---: | ---: | ---: |\\n");
    for bucket in &summary.score_buckets {
        let range = match (bucket.lower_bound, bucket.upper_bound) {
            (Some(lower), Some(upper)) => format!("{lower:.1}–{upper:.1}"),
            _ => "n/a".to_string(),
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} | {:.3} |\\n",
            bucket.bucket, range, bucket.total, bucket.correct, bucket.accuracy
        ));
    }

    out.push_str("\n## Reason counts\n\n| Reason | Count |\\n| --- | ---: |\\n");
    for (reason, count) in &summary.reason_counts {
        out.push_str(&format!("| {} | {} |\\n", reason, count));
    }

    if let Some(advanced) = &summary.advanced {
        out.push_str("\n## Confidence intervals (95%)\\n\n");
        out.push_str(&format!(
            "Precision: {:.3}–{:.3}\\n\\nRecall: {:.3}–{:.3}\\n\\nF1: {:.3}–{:.3}\\n\\nIterations: {}\\n",
            advanced.precision_ci.0,
            advanced.precision_ci.1,
            advanced.recall_ci.0,
            advanced.recall_ci.1,
            advanced.f1_ci.0,
            advanced.f1_ci.1,
            advanced.bootstrap_iterations
        ));
    }

    out
}
