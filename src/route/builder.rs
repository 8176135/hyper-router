use crate::HandlerWithCaptures;
use crate::RouteWithCaptures;
use crate::Handler as NormalHandler;
use crate::Route;

pub struct RouteBuilder {
    route: Route,
}

pub struct RouteWithCapturesBuilder {
    route: RouteWithCaptures,
}

pub trait RouteBuilderImpls {
    type Rt: crate::RouteHelpers;
    type Handler;
    fn new(route: Self::Rt) -> Self;
    fn using(self, handler: Self::Handler) -> Self::Rt;
}

impl RouteBuilderImpls for RouteBuilder {
    type Rt = Route;
    type Handler = NormalHandler;

    fn new(route: Route) -> Self {
        RouteBuilder { route }
    }

    /// Completes the building process by taking the handler to process the request.
    ///
    /// Returns created route.
    fn using(mut self, handler: Self::Handler) -> Self::Rt {
        self.route.handler = handler;
        self.route
    }
}

impl RouteBuilderImpls for RouteWithCapturesBuilder {
    type Rt = RouteWithCaptures;
    type Handler = HandlerWithCaptures;

    fn new(route: Self::Rt) -> Self {
        RouteWithCapturesBuilder { route }
    }

    /// Completes the building process by taking the handler to process the request.
    ///
    /// Returns created route.
    fn using(mut self, handler: Self::Handler) -> Self::Rt {
        self.route.handler = handler;
        self.route
    }
}
