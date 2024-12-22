// src/modules/auth/middleware.rs

use crate::utils::jwt::validate_jwt;

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use once_cell::sync::Lazy;
use std::{
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
};

/// Initialize the JWT_SECRET once
static JWT_SECRET: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"));

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct JwtMiddleware;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct JwtMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        // Get the Authorization header
        let auth_header = req
            .headers()
            .get("Authorization")
            .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Authorization header"));

        let (http_request, payload) = req.into_parts();

        // Process the Authorization header
        let auth_str = auth_header.and_then(|header| {
            header
                .to_str()
                .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid Authorization header"))
        });

        let token = auth_str.and_then(|auth| {
            auth.strip_prefix("Bearer ")
                .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing Bearer prefix"))
        });

        let fut = async move {
            match token {
                Ok(token) => match validate_jwt(JWT_SECRET.as_str(), token) {
                    Ok(claims) => {
                        let req = ServiceRequest::from_parts(http_request, payload);
                        req.extensions_mut().insert(Arc::new(claims));
                        service.call(req).await
                    }
                    Err(_) => Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
                },
                Err(err) => Err(err),
            }
        };

        Box::pin(fut)
    }
}
