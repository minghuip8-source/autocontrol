use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutoError {
    // ==========================
    // Windows API
    // ==========================
    #[error("Windows API error: {0}")]
    Windows(String),

    #[error("Windows API call failed: {api}, code: {code}")]
    WindowsApi { api: String, code: u32 },

    // ==========================
    // Keyboard
    // ==========================
    #[error("Invalid keyboard key: {0}")]
    InvalidKey(String),

    #[error("Keyboard operation failed: {0}")]
    Keyboard(String),

    // ==========================
    // Mouse
    // ==========================
    #[error("Mouse operation failed: {0}")]
    Mouse(String),

    #[error("Invalid mouse parameter: {0}")]
    InvalidMouseParameter(String),

    // ==========================
    // Screen
    // ==========================
    #[error("Screenshot capture failed")]
    ScreenshotFailed(String),

    #[error("Screen initialization failed: {0}")]
    ScreenInit(String),

    #[error("DXGI error: {0}")]
    Dxgi(String),

    // ==========================
    // Clipboard
    // ==========================
    #[error("Clipboard operation failed: {0}")]
    Clipboard(String),

    #[error("Clipboard is empty")]
    ClipboardEmpty,

    // ==========================
    // Window
    // ==========================
    #[error("Window not found: {0}")]
    WindowNotFound(String),

    #[error("Window operation failed: {0}")]
    Window(String),

    // ==========================
    // Parameter
    // ==========================
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    // ==========================
    // File
    // ==========================
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    // ==========================
    // Serialization
    // ==========================
    #[error("Serialization error: {0}")]
    Serialization(String),

    // ==========================
    // Other
    // ==========================
    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, AutoError>;
