use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread::{self, JoinHandle};

pub struct TestHttpServer {
    pub base_url: String,
    request_handle: JoinHandle<String>,
}

impl TestHttpServer {
    pub fn finish(self) -> String {
        self.request_handle.join().expect("join test HTTP server")
    }
}

pub struct CompletedTestHttpRequest<T> {
    pub request: String,
    pub output: T,
}

pub fn serve_once(response: &'static str) -> TestHttpServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind test HTTP listener");
    let base_url = format!(
        "http://{}",
        listener.local_addr().expect("listener address")
    );

    let request_handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept test HTTP request");
        let mut request = Vec::new();
        let mut buffer = [0; 1024];
        loop {
            let bytes_read = stream.read(&mut buffer).expect("read test HTTP request");
            if bytes_read == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..bytes_read]);
            if request.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        stream
            .write_all(response.as_bytes())
            .expect("write test HTTP response");
        String::from_utf8_lossy(&request).into_owned()
    });

    TestHttpServer {
        base_url,
        request_handle,
    }
}

pub async fn complete_request<F, Fut, T>(
    response: &'static str,
    request: F,
) -> CompletedTestHttpRequest<T>
where
    F: FnOnce(String) -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let server = serve_once(response);
    let base_url = server.base_url.clone();
    let output = request(base_url.clone()).await;
    let request = server.finish();
    CompletedTestHttpRequest { request, output }
}
