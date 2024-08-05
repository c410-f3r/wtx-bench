use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Protocol {
    Http2Framework,
    WebSocket,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}

impl From<&str> for Protocol {
    fn from(from: &str) -> Self {
        match from {
            "http2-framework" => Self::Http2Framework,
            "web-socket" => Self::WebSocket,
            _ => panic!(),
        }
    }
}

impl From<&Protocol> for &'static str {
    fn from(from: &Protocol) -> Self {
        match from {
            Protocol::Http2Framework => "http2-framework",
            Protocol::WebSocket => "web-socket",
        }
    }
}
