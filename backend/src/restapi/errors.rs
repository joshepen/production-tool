use actix_web::HttpResponse;
use sqlx::Error as SqlxError;

pub fn sqlx_error_to_http(err: SqlxError) -> actix_web::HttpResponse {
    match err {
        SqlxError::RowNotFound => HttpResponse::NotFound().body("Resource not found"),

        SqlxError::Database(db_err) => match db_err.code().as_deref() {
            Some("1062") => HttpResponse::Conflict().body("Resource already exists"),

            Some("1451") | Some("1452") => {
                HttpResponse::BadRequest().body("Invalid foreign key reference")
            }

            Some("23000") => HttpResponse::BadRequest()
                .body("Dependency restriction: other rows depend on this row."),

            _ => HttpResponse::InternalServerError().body("Database error"),
        },

        SqlxError::PoolTimedOut | SqlxError::PoolClosed | SqlxError::Io(_) => {
            HttpResponse::ServiceUnavailable().body("Database unavailable")
        }

        _ => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
