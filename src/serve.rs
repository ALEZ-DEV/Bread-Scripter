use std::{
    io::Read,
    path::{Path, PathBuf},
};

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use tokio::{
    fs::{self, File},
    io::AsyncReadExt,
};

use crate::config;

pub async fn serve_dir(
    request: Request<Body>,
    base_dir: PathBuf,
    mislead: config::Mislead,
) -> impl IntoResponse {
    let uri_path = request.uri().path().trim_start_matches("/");
    let file_path = base_dir.join(uri_path);

    if !file_path.exists() || file_path.is_dir() {
        return (StatusCode::NOT_FOUND, "File not found").into_response();
    }

    tracing::debug!("Serving \t-> {}", file_path.to_str().unwrap());

    if let Some(mime_type) = is_image_file(file_path.clone()).await.unwrap_or(None) {
        match fs::read(file_path).await {
            Ok(content) => Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", mime_type)
                .body(Body::from(content))
                .unwrap(),
            Err(err) => {
                tracing::error!("Failed to read file : {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error reading file").into_response()
            }
        }
    } else {
        match fs::read_to_string(file_path).await {
            Ok(content) => {
                let edited_content = content.replace(&mislead.link_to_mislead, &mislead.mislead_to);

                Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-type", "application/octet-stream")
                    .body(Body::from(edited_content))
                    .unwrap()
            }
            Err(err) => {
                tracing::error!("Failed to read file : {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Error reading file").into_response()
            }
        }
    }
}

async fn is_image_file(path: impl AsRef<Path>) -> anyhow::Result<Option<String>> {
    let kind = infer::get_from_path(path)?;

    if let Some(file) = kind {
        if file.mime_type().starts_with("image") {
            Ok(Some(String::from(file.mime_type())))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
