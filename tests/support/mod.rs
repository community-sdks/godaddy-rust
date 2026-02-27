use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use godaddy_rust::error::ApiResult;
use godaddy_rust::http::{Request, Response, Transport};

#[derive(Default)]
pub struct MockTransport {
    requests: Mutex<Vec<Request>>,
    responses: Mutex<VecDeque<Response>>,
}

impl MockTransport {
    pub fn shared() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn push_response(&self, response: Response) {
        self.responses.lock().unwrap().push_back(response);
    }

    #[allow(dead_code)]
    pub fn request_count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }

    #[allow(dead_code)]
    pub fn request_at(&self, index: usize) -> Option<Request> {
        self.requests.lock().unwrap().get(index).cloned()
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn send(&self, request: Request) -> ApiResult<Response> {
        self.requests.lock().unwrap().push(request);
        Ok(self
            .responses
            .lock()
            .unwrap()
            .pop_front()
            .unwrap_or_else(|| Response::json(200, "{}")))
    }
}
