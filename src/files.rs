pub enum FileRequest {
    Index,
    Cats,
    Pic,
    Style,
}

impl FileRequest {
    pub fn from_path(path: &str) -> Option<FileRequest> {
        match path {
            "/" => Some(FileRequest::Index),
            "/cats" => Some(FileRequest::Cats),
            "/images/pic.jpg" => Some(FileRequest::Pic),
            "/styles/style.css" => Some(FileRequest::Style),
            _ => None,
        }
    }

    pub fn get_filename(&self) -> &'static str {
        match self {
            FileRequest::Index => "webpages/index.html",
            FileRequest::Cats => "webpages/cats.html",
            FileRequest::Pic => "webpages/images/pic.jpg",
            FileRequest::Style => "webpages/styles/style.css",
        }
    }
}
