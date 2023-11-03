
pub enum Usage {
    VisionApi
}

impl Usage {
    pub(crate) fn as_str(&self) -> String {
        match self {
            Usage::VisionApi => String::from("https://www.googleapis.com/auth/cloud-vision"),
        }
    }
}