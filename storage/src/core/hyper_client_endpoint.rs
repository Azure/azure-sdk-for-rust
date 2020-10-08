use crate::ClientEndpoint;
use hyper_rustls::HttpsConnector;

pub trait HyperClientEndpoint: ClientEndpoint {
    fn hyper_client(&self) -> &hyper::Client<HttpsConnector<hyper::client::HttpConnector>>;
}
