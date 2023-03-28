pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
}

impl Request {
    pub fn new(request_data: &[u8]) -> Request {
        let mut method: &str = "";
        let mut path: &str = "";
        let mut version: &str = "";

        let data_string = String::from_utf8_lossy(&request_data);

        let lines = data_string.lines().collect::<Vec<&str>>();
        for line in lines {
            (method, path, version) = Self::parse_path(&line);
            break;
        }
        println!("\nData string:\n{}\n", &data_string);

        Request {
            method: String::from(method),
            path: String::from(path),
            version: String::from(version),
        }
    }

    fn parse_path(path_line: &str) -> (&str, &str, &str) {
        let mut split_line = path_line.split(' ');
        let method = split_line.next().unwrap().clone();
        let path = split_line.next().unwrap().clone();
        let version = split_line.next().unwrap().clone();

        (method, path, version)
    }
}
