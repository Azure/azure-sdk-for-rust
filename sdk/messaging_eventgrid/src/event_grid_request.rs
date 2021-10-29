use crate::event_grid_response::EventGridResponse;

pub struct EventGridRequest {
    request: Request<Body>,
}

impl EventGridRequest {
    pub fn request(self, client: &Client<HttpsConnector<HttpConnector>>) -> EventGridResponse {
        client.request(self.request).into()
    }
}

impl From<Request<Body>> for EventGridRequest {
    fn from(request: Request<Body>) -> EventGridRequest {
        EventGridRequest { request }
    }
}
