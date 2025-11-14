use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
// Core
use dfps_api::{router as api_router, ApiState};
use dfps_core::{
    mapping::{DimNCITConcept, MappingResult, MappingState},
    staging::{StgServiceRequestFlat, StgSrCodeExploded},
};
use dfps_observability::PipelineMetrics;
use dfps_test_suite::regression;

use http_body_util::BodyExt;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle};
use tower::ServiceExt;
use reqwest::StatusCode as ReqwestStatusCode;

#[derive(Deserialize)]
struct MapBundlesBody {
    flats: Vec<StgServiceRequestFlat>,
    exploded_codes: Vec<StgSrCodeExploded>,
    mapping_results: Vec<MappingResult>,
    dim_concepts: Vec<DimNCITConcept>,
}

#[derive(Deserialize)]
struct HealthResponse {
    status: String,
}

fn app() -> Router {
    api_router(ApiState::default())
}

async fn spawn_http_server() -> (SocketAddr, oneshot::Sender<()>, JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test server");
    let addr = listener.local_addr().expect("server addr");
    let router = api_router(ApiState::default());
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let handle = tokio::spawn(async move {
        let shutdown = async {
            let _ = shutdown_rx.await;
        };
        if let Err(err) = axum::serve(listener, router.into_make_service())
            .with_graceful_shutdown(shutdown)
            .await
        {
            panic!("dfps_api server error: {err}");
        }
    });

    (addr, shutdown_tx, handle)
}

async fn send_json<T>(app: &Router, request: Request<Body>) -> (StatusCode, T)
where
    T: DeserializeOwned,
{
    let response = app
        .clone()
        .oneshot(request)
        .await
        .expect("router responded");
    let status = response.status();
    let bytes = BodyExt::collect(response.into_body())
        .await
        .expect("collect response body")
        .to_bytes();
    let body = serde_json::from_slice(&bytes).expect("valid JSON body");
    (status, body)
}

#[tokio::test]
async fn map_bundles_returns_mapped_results() {
    let app = app();
    let bundle = regression::baseline_fhir_bundle();
    let payload = serde_json::to_vec(&bundle).expect("serialize bundle");

    let request = Request::builder()
        .method("POST")
        .uri("/api/map-bundles")
        .header("content-type", "application/json")
        .body(Body::from(payload))
        .expect("request body");

    let (status, body): (StatusCode, MapBundlesBody) = send_json(&app, request).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.flats.len(), 1);
    assert_eq!(body.exploded_codes.len(), 2);
    assert_eq!(body.mapping_results.len(), 2);
    assert!(
        body.mapping_results
            .iter()
            .any(|result| result.state == MappingState::AutoMapped)
    );
}

#[tokio::test]
async fn map_bundles_unknown_code_surfaces_no_match() {
    let app = app();
    let bundle = regression::fhir_bundle_unknown_code();
    let payload = serde_json::to_vec(&bundle).expect("serialize bundle");

    let request = Request::builder()
        .method("POST")
        .uri("/api/map-bundles")
        .header("content-type", "application/json")
        .body(Body::from(payload))
        .expect("request body");

    let (status, body): (StatusCode, MapBundlesBody) = send_json(&app, request).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.flats.len(), 1);
    assert_eq!(body.mapping_results.len(), 1);
    let result = &body.mapping_results[0];
    assert_eq!(result.state, MappingState::NoMatch);
    assert_eq!(result.reason.as_deref(), Some("missing_system_or_code"));
}

#[tokio::test]
async fn metrics_summary_tracks_processed_bundles() {
    let app = app();
    let bundle = regression::baseline_fhir_bundle();
    let payload = serde_json::to_vec(&bundle).expect("serialize bundle");

    let map_request = Request::builder()
        .method("POST")
        .uri("/api/map-bundles")
        .header("content-type", "application/json")
        .body(Body::from(payload))
        .expect("request body");

    let (status, _): (StatusCode, MapBundlesBody) = send_json(&app, map_request).await;
    assert_eq!(status, StatusCode::OK);

    let metrics_request = Request::builder()
        .method("GET")
        .uri("/metrics/summary")
        .body(Body::empty())
        .expect("metrics request");
    let (metrics_status, metrics): (StatusCode, PipelineMetrics) =
        send_json(&app, metrics_request).await;
    assert_eq!(metrics_status, StatusCode::OK);
    assert_eq!(metrics.bundle_count, 1);
    assert_eq!(metrics.flats_count, 1);
    assert!(metrics.mapping_count >= 1);

    let health_request = Request::builder()
        .method("GET")
        .uri("/health")
        .body(Body::empty())
        .expect("health request");
    let (health_status, health): (StatusCode, HealthResponse) =
        send_json(&app, health_request).await;
    assert_eq!(health_status, StatusCode::OK);
    assert_eq!(health.status, "ok");
}

#[tokio::test]
async fn ci_smoke_server_runs_endpoints() {
    let (addr, shutdown_tx, handle) = spawn_http_server().await;
    let client = reqwest::Client::new();
    let base = format!("http://{addr}");

    let bundle = regression::baseline_fhir_bundle();
    let map_resp = client
        .post(format!("{base}/api/map-bundles"))
        .json(&bundle)
        .send()
        .await
        .expect("map-bundles response");
    assert_eq!(map_resp.status(), ReqwestStatusCode::OK);
    let body: MapBundlesBody = map_resp.json().await.expect("map body json");
    assert_eq!(body.flats.len(), 1);
    assert!(!body.mapping_results.is_empty());

    let metrics_resp = client
        .get(format!("{base}/metrics/summary"))
        .send()
        .await
        .expect("metrics response");
    assert_eq!(metrics_resp.status(), ReqwestStatusCode::OK);
    let metrics: PipelineMetrics = metrics_resp.json().await.expect("metrics body");
    assert!(metrics.bundle_count >= 1);

    let health_resp = client
        .get(format!("{base}/health"))
        .send()
        .await
        .expect("health response");
    assert_eq!(health_resp.status(), ReqwestStatusCode::OK);
    let health: HealthResponse = health_resp.json().await.expect("health body");
    assert_eq!(health.status, "ok");

    let _ = shutdown_tx.send(());
    handle.await.expect("server join");
}
