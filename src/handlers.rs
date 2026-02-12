use axum::{extract::Path, http::StatusCode, response::Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::models::{Comment, Review, ReviewStatus};

pub async fn get_reviews() -> Result<Json<Value>, StatusCode> {
    // Mock data for initial implementation
    let reviews = vec![
        Review {
            id: Uuid::new_v4(),
            title: "Add user authentication system".to_string(),
            description: Some("Implement JWT-based auth with refresh tokens".to_string()),
            author_id: Uuid::new_v4(),
            status: ReviewStatus::Open,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        Review {
            id: Uuid::new_v4(),
            title: "Optimize database queries".to_string(),
            description: Some("Add indexes and query optimization".to_string()),
            author_id: Uuid::new_v4(),
            status: ReviewStatus::Draft,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
    ];
    
    Ok(Json(json!({
        "reviews": reviews,
        "total": reviews.len()
    })))
}

pub async fn get_comments(Path(review_id): Path<Uuid>) -> Result<Json<Value>, StatusCode> {
    let comments = vec![
        Comment {
            id: Uuid::new_v4(),
            review_id,
            author_id: Uuid::new_v4(),
            content: "This looks good, but consider adding error handling here.".to_string(),
            line_number: Some(42),
            file_path: Some("src/auth.rs".to_string()),
            parent_id: None,
            created_at: chrono::Utc::now(),
        },
    ];
    
    Ok(Json(json!({
        "comments": comments,
        "total": comments.len()
    })))
}