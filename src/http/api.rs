use axum::{routing::post, Router};

pub fn router() -> Router {
    Router::new().route("/links", post(handlers::create_link))
}

mod handlers {
    use std::sync::Arc;

    use axum::{
        extract::{Extension, Json},
        http::StatusCode,
    };
    use serde::Deserialize;
    use tracing::{debug, instrument};
    use url::Url;

    use crate::http::State;

    #[derive(Deserialize, Debug)]
    pub struct CreateLink {
        from: Url,
        to: Url,
    }

    #[instrument]
    pub(super) async fn create_link(
        Json(link): Json<CreateLink>,
        Extension(state): Extension<Arc<State>>,
    ) -> (StatusCode, String) {
        debug!("Creating new link");

        let result = sqlx::query(
            r#"
INSERT INTO redirekt_links (host, path, destination)
VALUES (?1, ?2, ?3)
            "#,
        )
        .bind(link.from.host_str())
        .bind(link.from.path())
        .bind(link.to.to_string())
        .execute(&state.pool)
        .await;

        debug!(?result);

        (StatusCode::NOT_IMPLEMENTED, "".to_string())
    }
}
