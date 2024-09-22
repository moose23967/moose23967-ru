use may_minihttp::{HttpService, Request, Response};
use std::collections::HashMap;
use std::fs::read;
use std::fs::read_dir;

#[derive(Clone)]
pub struct Application;

impl HttpService for Application {
    fn call(&mut self, request: Request, response: &mut Response) -> std::io::Result<()> {
        let mut paths: HashMap<String, Vec<u8>> = HashMap::new();

        for dir_entry in read_dir("src/static")? {
            let dir_entry = dir_entry?;

            let file_name = dir_entry.file_name().into_string().unwrap();
            let content = read(dir_entry.path())?;

            paths.insert(format!("/{file_name}"), content);
        }

        paths.insert(String::from("/"), read("src/static/index.html")?);

        let request_path = request.path();

        if request_path == "/" {
            response.header("Content-Type: text/html");
        }

        let vector = paths.get(request.path()).unwrap().to_vec();

        response.body_vec(vector);
        response.status_code(200, "OK");

        Ok(())
    }
}
