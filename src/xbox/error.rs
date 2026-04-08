#[derive(Debug)]
pub enum GamePadError {
    DeviceOpen(std::io::Error),
    Read(std::io::Error),
}

impl std::fmt::Display for GamePadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GamePadError::DeviceOpen(e) => write!(f, "failed to open device: {}", e),
            GamePadError::Read(e) => write!(f, "failed to read device: {}", e),
        }
    }
}

impl std::error::Error for GamePadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GamePadError::DeviceOpen(e) => Some(e),
            GamePadError::Read(e) => Some(e),
        }
    }
}
