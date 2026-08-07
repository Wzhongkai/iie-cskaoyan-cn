use crate::{error::ApiError, models::UploadReceipt, state::AppState};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use std::path::PathBuf;
use tokio::fs;
use tracing::info;
use uuid::Uuid;

pub(crate) fn detect_image_extension(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]) {
        Some("png")
    } else if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        Some("jpg")
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        Some("gif")
    } else if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        Some("webp")
    } else {
        None
    }
}

pub(crate) async fn upload_directory_size(path: &PathBuf) -> Result<u64, std::io::Error> {
    let mut total = 0;
    let mut entries = fs::read_dir(path).await?;
    while let Some(entry) = entries.next_entry().await? {
        let metadata = entry.metadata().await?;
        if metadata.is_file() {
            total += metadata.len();
        }
    }
    Ok(total)
}

pub(crate) async fn upload_image(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<UploadReceipt>), ApiError> {
    const MAX_IMAGE_BYTES: usize = 5 * 1024 * 1024;
    const MAX_UPLOAD_STORAGE_BYTES: u64 = 512 * 1024 * 1024;

    let mut uploaded = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::BadRequest("无法读取上传内容".into()))?
    {
        if field.name() != Some("image") {
            continue;
        }
        let original_name = field.file_name().unwrap_or("image").to_owned();
        let bytes = field
            .bytes()
            .await
            .map_err(|_| ApiError::BadRequest("无法读取图片文件".into()))?;
        if bytes.is_empty() || bytes.len() > MAX_IMAGE_BYTES {
            return Err(ApiError::BadRequest("单张图片应小于 5 MB".into()));
        }
        let extension = detect_image_extension(&bytes)
            .ok_or_else(|| ApiError::BadRequest("仅支持 JPEG、PNG、WebP 和 GIF 图片".into()))?;
        uploaded = Some((original_name, bytes, extension));
        break;
    }

    let (original_name, bytes, extension) =
        uploaded.ok_or_else(|| ApiError::BadRequest("请选择需要上传的图片".into()))?;
    let current_size = upload_directory_size(&state.upload_dir)
        .await
        .map_err(|error| ApiError::BadRequest(format!("无法检查上传目录: {error}")))?;
    if current_size + bytes.len() as u64 > MAX_UPLOAD_STORAGE_BYTES {
        return Err(ApiError::BadRequest(
            "图片存储空间已满，请联系维护者清理".into(),
        ));
    }

    let filename = format!("{}.{}", Uuid::new_v4().simple(), extension);
    fs::write(state.upload_dir.join(&filename), &bytes)
        .await
        .map_err(|error| ApiError::BadRequest(format!("图片保存失败: {error}")))?;
    info!(filename = %filename, original_name = %original_name, size = bytes.len(), "submission image uploaded");
    Ok((
        StatusCode::CREATED,
        Json(UploadReceipt {
            url: format!("/uploads/{filename}"),
            filename,
            size: bytes.len(),
        }),
    ))
}
