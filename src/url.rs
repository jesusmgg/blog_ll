const DOCUMENT_ROOT: &str = "documents/";

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
    Ok,
    NotFound,
}

pub fn get_status_code_str(status_code: &StatusCode) -> &'static str {
    match status_code {
        StatusCode::Ok => "200 OK",
        StatusCode::NotFound => "404 Not Found",
    }
}

pub fn handle_url(url: &str) -> Option<(String, StatusCode)> {
    for document in URLS {
        if document.url == url {
            let path = format!("{}{}", DOCUMENT_ROOT, document.document_path);
            return Some((path, StatusCode::Ok));
        }
    }

    let path = format!("{}{}", DOCUMENT_ROOT, NOT_FOUND_DOCUMENT_PATH);
    return Some((path, StatusCode::NotFound));
}
