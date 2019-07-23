use crate::handlers;
use hyper::Method;
use std::fmt;

use super::RouteBuilder;
use super::RouteWithCapturesBuilder;
use super::RouteBuilderImpls;
use crate::{Handler, HandlerWithCaptures};
use crate::Path;

/// Holds route information
pub struct Route {
    /// HTTP method to match
    pub method: Method,

    /// Path to match
    pub path: Path,

    /// Request handler
    ///
    /// This should be method that accepts Hyper's Request and Response:
    ///
    /// ```ignore
    /// use hyper::server::{Request, Response};
    /// use hyper::header::{ContentLength, ContentType};
    ///
    /// fn hello_handler(_: Request) -> Response {
    ///     let body = "Hello World";
    ///     Response::new()
    ///         .with_header(ContentLength(body.len() as u64))
    ///         .with_header(ContentType::plaintext())
    ///         .with_body(body)
    /// }
    /// ```
    pub handler: Handler,
}

pub struct RouteWithCaptures {
        /// HTTP method to match
    pub method: Method,

    /// Path to match
    pub path: Path,

    /// Request with captures handler 
    ///
    /// This should be method that accepts Hyper's Request and Response:
    ///
    /// ```ignore
    /// use hyper::server::{Request, Response};
    /// use hyper::header::{ContentLength, ContentType};
    ///
    /// fn hello_handler(_: Request, _: Captures) -> Response {
    ///     let body = "Hello World";
    ///     Response::new()
    ///         .with_header(ContentLength(body.len() as u64))
    ///         .with_header(ContentType::plaintext())
    ///         .with_body(body)
    /// }
    /// ```
    pub handler: HandlerWithCaptures,
}

pub trait RouteHelpers {

    type RtBuilder: RouteBuilderImpls;

    fn options(path: &str) -> Self::RtBuilder {
        Self::builder(Method::OPTIONS, path)
    }

    fn get(path: &str) -> Self::RtBuilder {
        Self::builder(Method::GET, path)
    }

    fn post(path: &str) -> Self::RtBuilder {
        Self::builder(Method::POST, path)
    }

    fn put(path: &str) -> Self::RtBuilder {
        Self::builder(Method::PUT, path)
    }

    fn delete(path: &str) -> Self::RtBuilder {
        Self::builder(Method::DELETE, path)
    }

    fn head(path: &str) -> Self::RtBuilder {
        Self::builder(Method::HEAD, path)
    }

    fn trace(path: &str) -> Self::RtBuilder {
        Self::builder(Method::TRACE, path)
    }

    fn connect(path: &str) -> Self::RtBuilder {
        Self::builder(Method::CONNECT, path)
    }

    fn patch(path: &str) -> Self::RtBuilder {
        Self::builder(Method::PATCH, path)
    }

    fn builder(method: Method, path: &str) -> Self::RtBuilder;
}

impl RouteHelpers for Route {
    type RtBuilder = RouteBuilder;

    fn builder(method: Method, path: &str) -> Self::RtBuilder {
        Self::RtBuilder::new(Route {
            method,
            path: Path::new(path),
            ..Route::default()
        })
    }
}

impl RouteHelpers for RouteWithCaptures {
    type RtBuilder = RouteWithCapturesBuilder;

    fn builder(method: Method, path: &str) -> Self::RtBuilder {
        Self::RtBuilder::new(RouteWithCaptures {
            method,
            path: Path::new(path),
            ..RouteWithCaptures::default()
        })
    }
}

impl Default for Route {
    fn default() -> Route {
        Route {
            method: Method::GET,
            path: Path::new("/"),
            handler: handlers::not_implemented_handler,
        }
    }
}

impl Default for RouteWithCaptures {
    fn default() -> RouteWithCaptures {
        RouteWithCaptures {
            method: Method::GET,
            path: Path::new("/"),
            handler: handlers::not_implemented_handler_captures,
        }
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Route {{method: {:?}, path: {:?}}}",
            self.method, self.path
        )
    }
}
