use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

// Use ammonia for HTML sanitization similar to Jsoup+Safelist
// Cargo.toml:
// ammonia = "3"
fn sanitize_str(s: &str) -> String {
    // Customize the whitelist here if needed to mimic CustomSafelist
    ammonia::Builder::default()
        .clean_content_tags(true)
        .clean(s)
}

fn sanitize_json_value(v: &mut Value) {
    match v {
        Value::String(s) => {
            let cleaned = sanitize_str(s);
            *s = cleaned;
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                sanitize_json_value(item);
            }
        }
        Value::Object(map) => {
            for (_, val) in map.iter_mut() {
                sanitize_json_value(val);
            }
        }
        _ => {}
    }
}

pub struct JsonSanitized<T>(pub T);

impl<T> JsonSanitized<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[derive(Serialize)]
struct ErrorBody {
    code: i32,
    msg: String,
    data: Option<Value>,
}

impl IntoResponse for ErrorBody {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, axum::Json(self)).into_response()
    }
}

#[async_trait]
impl<S, T> FromRequest<S> for JsonSanitized<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Debug,
{
    type Rejection = Response;

    async fn from_request(
        mut req: axum::http::Request<axum::body::Body>,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Ensure Content-Type is application/json like XssHttpMessageConverter
        let is_json = req
            .headers()
            .get(axum::http::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .map(|ct| ct.starts_with("application/json"))
            .unwrap_or(false);

        if !is_json {
            return Err(ErrorBody {
                code: 400,
                msg: "Content-Type must be application/json".to_string(),
                data: None,
            }
            .into_response());
        }

        let bytes = hyper::body::to_bytes(req.body_mut()).await.map_err(|e| {
            ErrorBody {
                code: 400,
                msg: format!("Failed to read body: {}", e),
                data: None,
            }
            .into_response()
        })?;

        let mut val: Value = serde_json::from_slice(&bytes).map_err(|e| {
            ErrorBody {
                code: 400,
                msg: format!("Failed to parse JSON data: {}", e),
                data: None,
            }
            .into_response()
        })?;

        sanitize_json_value(&mut val);

        let t: T = serde_json::from_value(val).map_err(|e| {
            ErrorBody {
                code: 400,
                msg: format!("Failed to map JSON to target type: {}", e),
                data: None,
            }
            .into_response()
        })?;

        Ok(JsonSanitized(t))
    }
}
