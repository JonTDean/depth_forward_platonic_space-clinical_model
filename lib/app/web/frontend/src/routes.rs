use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse, Result, web};
use bytes::BytesMut;
use futures_util::TryStreamExt;
use serde::Deserialize;

use crate::{
    client::{BackendClient, ClientError},
    state::AppState,
    view_model::{AlertKind, AlertMessage, HealthOverview, MappingResultsView, PageContext},
    views,
};

const MAX_UPLOAD_BYTES: usize = 512 * 1024; // 512KiB bundles are enough for MVP use.

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)))
        .service(web::resource("/map/paste").route(web::post().to(map_from_paste)))
        .service(web::resource("/map/upload").route(web::post().to(map_from_upload)));
}

async fn index(state: web::Data<AppState>) -> Result<HttpResponse> {
    let ctx = build_base_context(&state.client).await;
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(views::render_page(&ctx)))
}

#[derive(Deserialize)]
struct BundleForm {
    bundle_text: String,
}

async fn map_from_paste(
    state: web::Data<AppState>,
    req: HttpRequest,
    form: web::Form<BundleForm>,
) -> Result<HttpResponse> {
    let hx = is_htmx(&req);
    let mut ctx = build_base_context(&state.client).await;
    let trimmed = form.bundle_text.trim();
    if trimmed.is_empty() {
        ctx.alert = Some(AlertMessage {
            kind: AlertKind::Error,
            text: "Paste a Bundle payload before submitting.".to_string(),
        });
        return Ok(respond(ctx, hx));
    }

    match serde_json::from_str::<serde_json::Value>(trimmed) {
        Ok(value) => handle_mapping(value, state, ctx, hx).await,
        Err(err) => {
            ctx.alert = Some(AlertMessage {
                kind: AlertKind::Error,
                text: format!("Invalid JSON: {err}"),
            });
            Ok(respond(ctx, hx))
        }
    }
}

async fn map_from_upload(
    state: web::Data<AppState>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse> {
    let hx = is_htmx(&req);
    let mut ctx = build_base_context(&state.client).await;
    match read_bundle_file(&mut payload).await {
        Ok(Some(text)) => match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(value) => handle_mapping(value, state, ctx, hx).await,
            Err(err) => {
                ctx.alert = Some(AlertMessage {
                    kind: AlertKind::Error,
                    text: format!("Invalid JSON: {err}"),
                });
                Ok(respond(ctx, hx))
            }
        },
        Ok(None) => {
            ctx.alert = Some(AlertMessage {
                kind: AlertKind::Error,
                text: "Choose a JSON file before submitting.".to_string(),
            });
            Ok(respond(ctx, hx))
        }
        Err(msg) => {
            ctx.alert = Some(AlertMessage {
                kind: AlertKind::Error,
                text: msg,
            });
            Ok(respond(ctx, hx))
        }
    }
}

async fn handle_mapping(
    payload: serde_json::Value,
    state: web::Data<AppState>,
    mut ctx: PageContext,
    hx: bool,
) -> Result<HttpResponse> {
    match state.client.map_bundles(payload).await {
        Ok(response) => {
            ctx.results = Some(MappingResultsView::from_response(&response));
            ctx.alert = Some(AlertMessage {
                kind: AlertKind::Info,
                text: format!(
                    "Mapped {} codes",
                    ctx.results.as_ref().map(|res| res.rows.len()).unwrap_or(0)
                ),
            });
            Ok(respond(ctx, hx))
        }
        Err(err) => {
            ctx.alert = Some(AlertMessage {
                kind: AlertKind::Error,
                text: format!("Backend error: {}", summarize_client_error(err)),
            });
            Ok(respond(ctx, hx))
        }
    }
}

async fn build_base_context(client: &BackendClient) -> PageContext {
    let health = client.health().await.ok().map(|resp| {
        let status = resp.status;
        let ok = status == "ok";
        HealthOverview { status, ok }
    });
    let metrics = client.metrics_summary().await.ok();
    PageContext {
        health,
        metrics,
        ..Default::default()
    }
}

fn respond(ctx: PageContext, hx: bool) -> HttpResponse {
    if hx {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(views::render_results_fragment(&ctx))
    } else {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(views::render_page(&ctx))
    }
}

fn is_htmx(req: &HttpRequest) -> bool {
    req.headers().contains_key("HX-Request")
}

async fn read_bundle_file(payload: &mut Multipart) -> Result<Option<String>, String> {
    while let Some(field) = payload
        .try_next()
        .await
        .map_err(|err| format!("Failed to read upload: {err}"))?
    {
        if field.name() != "bundle_file" {
            continue;
        }
        let mut field = field;
        let mut bytes = BytesMut::new();
        while let Some(chunk) = field
            .try_next()
            .await
            .map_err(|err| format!("Failed to read upload chunk: {err}"))?
        {
            if bytes.len() + chunk.len() > MAX_UPLOAD_BYTES {
                return Err("Uploaded file is too large (max 512KiB)".to_string());
            }
            bytes.extend_from_slice(&chunk);
        }
        if bytes.is_empty() {
            return Err("Uploaded file is empty".to_string());
        }
        return String::from_utf8(bytes.to_vec())
            .map(Some)
            .map_err(|_| "File must be UTF-8 encoded JSON".to_string());
    }

    Ok(None)
}

fn summarize_client_error(err: ClientError) -> String {
    match err {
        ClientError::Backend { status, body } => {
            format!("status {status} – {body}")
        }
        ClientError::Http(inner) => format!("HTTP error: {inner}"),
        ClientError::InvalidJson(inner) => format!("Invalid JSON: {inner}"),
        ClientError::Upload(msg) => msg,
        ClientError::Utf8(inner) => format!("UTF-8 error: {inner}"),
        ClientError::EmptyBundle => "No bundle payload supplied".to_string(),
    }
}
