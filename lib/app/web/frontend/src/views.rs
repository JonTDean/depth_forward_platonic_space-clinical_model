use dfps_core::mapping::MappingState;
use maud::{DOCTYPE, Markup, html};

use crate::view_model::{AlertKind, AlertMessage, MappingResultsView, PageContext};

pub fn render_page(ctx: &PageContext) -> String {
    html! {
        (DOCTYPE)
        html class="h-full bg-slate-100" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "DFPS Mapping Workbench" }
                script src="https://cdn.tailwindcss.com" {}
                script src="https://unpkg.com/htmx.org@1.9.12" {}
            }
            body class="min-h-screen bg-slate-100 text-slate-900" {
                main class="mx-auto max-w-6xl px-4 py-10 space-y-8" {
                    section class="bg-white shadow-sm rounded-xl p-6" {
                        h1 class="text-2xl font-semibold" { "FHIR → NCIt mapping" }
                        p class="text-slate-600 mt-2" {
                            "Paste a FHIR Bundle or upload a JSON file to see how the DFPS pipeline maps SR codes into NCIt concepts."
                        }
                        div class="mt-4 flex flex-wrap gap-4 text-sm" {
                            @if let Some(health) = &ctx.health {
                                span class={(format!("inline-flex items-center gap-2 rounded-full px-3 py-1 text-xs font-medium {}",
                                    if health.ok { "bg-emerald-100 text-emerald-800" } else { "bg-amber-100 text-amber-800" }
                                ))} {
                                    span class={(if health.ok { "h-2 w-2 rounded-full bg-emerald-500" } else { "h-2 w-2 rounded-full bg-amber-500" })} {}
                                    span { (format!("Backend health: {}", health.status)) }
                                }
                            } @else {
                                span class="inline-flex items-center rounded-full bg-amber-100 px-3 py-1 text-xs font-medium text-amber-800" {
                                    "Backend health unknown"
                                }
                            }
                            @if let Some(metrics) = &ctx.metrics {
                                span class="inline-flex items-center rounded-full bg-slate-100 px-3 py-1 text-xs text-slate-700" {
                                    (format!("Bundles processed: {} | AutoMapped: {} | NeedsReview: {} | NoMatch: {}",
                                        metrics.bundle_count,
                                        metrics.auto_mapped,
                                        metrics.needs_review,
                                        metrics.no_match
                                    ))
                                }
                            }
                        }
                    }
                    section class="grid gap-6 lg:grid-cols-2" {
                        div class="bg-white shadow-sm rounded-xl p-6" {
                            h2 class="text-lg font-semibold" { "Paste Bundle JSON" }
                            form hx-post="/map/paste" hx-target="#results" hx-swap="innerHTML" method="post" class="mt-4 space-y-4" {
                                label class="block text-sm font-medium text-slate-700" for="bundle_text" {
                                    "FHIR Bundle JSON"
                                }
                                textarea name="bundle_text" id="bundle_text" rows="10" class="w-full rounded-lg border border-slate-300 p-3 font-mono text-sm focus:border-emerald-500 focus:ring-emerald-200" placeholder="{ \"resourceType\": \"Bundle\", ... }" {}
                                button type="submit" class="inline-flex items-center rounded-lg bg-emerald-600 px-4 py-2 text-white font-medium hover:bg-emerald-700" {
                                    "Map bundle"
                                }
                            }
                        }
                        div class="bg-white shadow-sm rounded-xl p-6" {
                            h2 class="text-lg font-semibold" { "Upload Bundle JSON" }
                            form hx-post="/map/upload" hx-target="#results" hx-swap="innerHTML" method="post" enctype="multipart/form-data" class="mt-4 space-y-4" {
                                label class="block text-sm font-medium text-slate-700" for="bundle_file" {
                                    "JSON file"
                                }
                                input type="file" id="bundle_file" name="bundle_file" accept="application/json,.json,.ndjson" class="w-full rounded-lg border border-dashed border-slate-300 p-3 text-sm" {}
                                button type="submit" class="inline-flex items-center rounded-lg bg-slate-800 px-4 py-2 text-white font-medium hover:bg-slate-900" {
                                    "Upload & map"
                                }
                            }
                        }
                    }
                    section id="results" class="space-y-4" {
                        (render_results(ctx))
                    }
                }
            }
        }
    }
    .into_string()
}

pub fn render_results_fragment(ctx: &PageContext) -> String {
    render_results(ctx).into_string()
}

fn render_results(ctx: &PageContext) -> Markup {
    html! {
        @if let Some(alert) = &ctx.alert {
            (render_alert(alert))
        }
        @if let Some(results) = &ctx.results {
            (render_results_panel(results))
        } @else {
            div class="bg-white rounded-xl border border-dashed border-slate-200 p-6 text-center text-slate-500" {
                p { "Results will appear here after the backend maps your bundle." }
                p class="text-sm mt-2" { "Supports a single Bundle object, an array, or NDJSON payloads." }
            }
        }
    }
}

fn render_alert(alert: &AlertMessage) -> Markup {
    let (bg, text) = match alert.kind {
        AlertKind::Info => ("bg-emerald-50 text-emerald-900", "Info"),
        AlertKind::Error => ("bg-rose-50 text-rose-900", "Error"),
    };
    html! {
        div class={(format!("rounded-lg px-4 py-3 text-sm font-medium {bg}"))} {
            strong class="mr-2" { (text) ":" }
            span { (&alert.text) }
        }
    }
}

fn render_results_panel(results: &MappingResultsView) -> Markup {
    html! {
        div class="bg-white shadow rounded-xl p-6 space-y-6" {
            h2 class="text-xl font-semibold" { "Mapping results" }
            div class="grid gap-4 md:grid-cols-3" {
                div class="rounded-lg border border-slate-200 p-4" {
                    p class="text-sm text-slate-500" { "SR flats" }
                    p class="text-2xl font-semibold" { (results.request_summary.total) }
                }
                div class="rounded-lg border border-slate-200 p-4" {
                    p class="text-sm font-semibold text-slate-600" { "Statuses" }
                    @if results.request_summary.statuses.is_empty() {
                        p class="text-sm text-slate-500" { "No statuses returned" }
                    } @else {
                        ul class="mt-2 space-y-1" {
                            @for stat in &results.request_summary.statuses {
                                li class="flex justify-between text-sm" {
                                    span { (&stat.label) }
                                    span class="font-medium" { (&stat.count) }
                                }
                            }
                        }
                    }
                }
                div class="rounded-lg border border-slate-200 p-4" {
                    p class="text-sm font-semibold text-slate-600" { "Intents" }
                    @if results.request_summary.intents.is_empty() {
                        p class="text-sm text-slate-500" { "No intents returned" }
                    } @else {
                        ul class="mt-2 space-y-1" {
                            @for stat in &results.request_summary.intents {
                                li class="flex justify-between text-sm" {
                                    span { (&stat.label) }
                                    span class="font-medium" { (&stat.count) }
                                }
                            }
                        }
                    }
                }
            }
            div class="overflow-x-auto" {
                table class="min-w-full divide-y divide-slate-200" {
                    thead class="bg-slate-50" {
                        tr {
                            th class="px-4 py-2 text-left text-xs font-semibold uppercase tracking-wide text-slate-600" { "SR" }
                            th class="px-4 py-2 text-left text-xs font-semibold uppercase tracking-wide text-slate-600" { "Code" }
                            th class="px-4 py-2 text-left text-xs font-semibold uppercase tracking-wide text-slate-600" { "NCIt concept" }
                            th class="px-4 py-2 text-left text-xs font-semibold uppercase tracking-wide text-slate-600" { "State" }
                        }
                    }
                    tbody class="divide-y divide-slate-100 bg-white" {
                        @if results.rows.is_empty() {
                            tr {
                                td colspan="4" class="px-4 py-6 text-center text-slate-500" {
                                    "No mapping results returned."
                                }
                            }
                        } @else {
                            @for row in &results.rows {
                                tr {
                                    td class="px-4 py-3 align-top" {
                                        p class="font-medium" { (&row.sr_id) }
                                        p class="text-sm text-slate-500" { (&row.system) }
                                    }
                                    td class="px-4 py-3 align-top" {
                                        p class="font-semibold" { (&row.code) }
                                        p class="text-sm text-slate-500" { (&row.display) }
                                    }
                                    td class="px-4 py-3 align-top" {
                                        @if let Some(id) = &row.ncit_id {
                                            p class="font-medium" { (id) }
                                            @if let Some(label) = &row.ncit_label {
                                                p class="text-sm text-slate-500" { (label) }
                                            }
                                        } @else {
                                            span class="text-slate-400" { "—" }
                                        }
                                    }
                                    td class="px-4 py-3 align-top" {
                                        (state_chip(row.state))
                                        @if let Some(reason) = &row.reason {
                                            p class="mt-1 text-xs text-slate-500" {
                                                "Reason: " (reason)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn state_chip(state: MappingState) -> Markup {
    let (label, classes) = match state {
        MappingState::AutoMapped => (
            "AutoMapped",
            "bg-emerald-100 text-emerald-900 ring-emerald-200",
        ),
        MappingState::NeedsReview => ("Needs review", "bg-amber-100 text-amber-900 ring-amber-200"),
        MappingState::NoMatch => ("No match", "bg-rose-100 text-rose-900 ring-rose-200"),
    };
    html! {
        span class={(format!("inline-flex items-center rounded-full px-3 py-1 text-xs font-semibold ring-1 ring-inset {classes}"))} {
            (label)
        }
    }
}
