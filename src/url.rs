struct DocumentUrl<'a> {
    url: &'a str,
    document_path: &'a str,
}

const URLS: [DocumentUrl; 2] = [
    DocumentUrl {
        url: "/home",
        document_path: "home.html",
    },
    DocumentUrl {
        url: "/blog",
        document_path: "blog.html",
    },
];

const NOT_FOUND_DOCUMENT_PATH: &str = "404.html";

#[derive(Debug)]
pub enum StatusCode {
    Success,
    NotFound,
}

pub fn handle_url(url: &str) -> Option<(&str, StatusCode)> {
    for document in URLS {
        if document.url == url {
            return Some((document.document_path, StatusCode::Success));
        }
    }

    return Some((NOT_FOUND_DOCUMENT_PATH, StatusCode::NotFound));
}
