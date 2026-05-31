#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Database not found")]
    DatabaseNotFound {
        path: String,
        #[source]
        source: anyhow::Error,
    },

    #[error("Invalid path")]
    InvalidPath {
        path: String,
        #[source]
        source: anyhow::Error,
    },

    #[error("CSV import failed")]
    CsvImportFailed {
        path: String,
        #[source]
        source: anyhow::Error,
    },

    #[error("CSV export failed")]
    CsvExportFailed {
        path: String,
        #[source]
        source: anyhow::Error,
    },

    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),

    #[error("IO error")]
    IOError(#[from] std::io::Error),

    #[error("Illegal action")]
    IllegalAction { action: String },

    #[error("Something went wrong")]
    Other(String),
}
