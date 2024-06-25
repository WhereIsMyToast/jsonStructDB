use core::fmt;

pub struct AppError {
    pub code: usize,
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_message = match self.code {
            4 => "Error reading from file",
            3 => "Error writing to file",
            2 => "Error opening file",
            1 => "Wrong identifier",
            _ => "Unidentified error",
        };

        write!(f, "{}", err_message)
    }
}
