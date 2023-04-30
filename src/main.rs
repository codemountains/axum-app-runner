use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let hc_router = Router::new().route("/", get(health_check));
    let mountain_router = Router::new().route("/", get(find_sacred_mountains));

    let app = Router::new()
        .nest("/hc", hc_router)
        .nest("/mountains", mountain_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn find_sacred_mountains() -> (StatusCode, Json<JsonResponse>) {
    let response: JsonResponse = Default::default();
    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonResponse {
    mountains: Vec<EightThousander>,
    total: usize,
}

impl Default for JsonResponse {
    fn default() -> Self {
        let mountains = vec![
            EightThousander::new(1, "エベレスト".to_string(), 8848),
            EightThousander::new(2, "K2".to_string(), 8611),
            EightThousander::new(3, "カンチェンジェンガ".to_string(), 8586),
            EightThousander::new(4, "ローツェ".to_string(), 8516),
            EightThousander::new(5, "マカルー".to_string(), 8463),
            EightThousander::new(6, "チョ・オユー".to_string(), 8188),
            EightThousander::new(7, "ダウラギリ".to_string(), 8167),
            EightThousander::new(8, "マナスル".to_string(), 8163),
            EightThousander::new(9, "ナンガ・パルバット".to_string(), 8126),
            EightThousander::new(10, "アンナプルナ".to_string(), 8091),
            EightThousander::new(11, "ガッシャーブルⅠ峰".to_string(), 8080),
            EightThousander::new(12, "ブロード・ピーク".to_string(), 8051),
            EightThousander::new(13, "ガッシャーブルムⅡ峰".to_string(), 8035),
            EightThousander::new(14, "シシャパンマ".to_string(), 8027),
        ];
        let total = mountains.len();

        Self { mountains, total }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EightThousander {
    id: i32,
    name: String,
    elevation: i32,
}

impl EightThousander {
    fn new(id: i32, name: String, elevation: i32) -> Self {
        Self {
            id,
            name,
            elevation,
        }
    }
}
