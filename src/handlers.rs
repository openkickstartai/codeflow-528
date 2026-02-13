use axum::{extract::Path, http::StatusCode, response::Json};
use serde::Serialize;
use uuid::Uuid;

use crate::models::{Comment, Review, ReviewStatus};

#[derive(Serialize)]
pub struct ReviewListResponse {
    reviews: Vec<Review>,
    total: usize,
}

#[derive(Serialize)]
pub struct CommentListResponse {
    comments: Vec<Comment>,
    total: usize,
}

pub async fn get_reviews() -> Result<Json<ReviewListResponse>, StatusCode> {
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
    
    let total = reviews.len();
    Ok(Json(ReviewListResponse { reviews, total }))
}

pub async fn get_comments(Path(review_id): Path<Uuid>) -> Result<Json<CommentListResponse>, StatusCode> {
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
    
    let total = comments.len();
    Ok(Json(CommentListResponse { comments, total }))
}