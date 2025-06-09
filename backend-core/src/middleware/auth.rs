use axum::{
    async_trait,
    extract::{FromRequest, TypedHeader},
    http::{Request, StatusCode},
    middleware,
    response::Response,
};
use axum_extra::extract::TypedHeader;
use headers::{authorization::Bearer, Authorization}; 