use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use dfps_api::{ApiState, router as api_router};
use dfps_core::{
    mapping::{DimNCITConcept, MappingResult, MappingState},
    staging::{StgServiceRequestFlat, StgSrCodeExploded},
};
use dfps_observability::PipelineMetrics;
use dfps_test_suite::regression;
use http_body_util::BodyExt;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use tower::ServiceExt;

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
