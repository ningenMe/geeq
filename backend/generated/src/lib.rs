#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use types::*;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "1.0.0";

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthLoginPostResponse {

    Status200
    {
        body: models::Common200Response,
        set_cookie:
        Option<
        String
        >
    }
    ,

    Status401
    (models::Common401Response)
    ,

    Status500
    (models::Common500Response)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthLogoutPostResponse {

    Status200
    (models::Common200Response)
    ,

    Status401
    (models::Common401Response)
    ,

    Status500
    (models::Common500Response)
}

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum AuthMeGetResponse {

    Status200
    (models::AuthMeGet200Response)
    ,

    Status401
    (models::Common401Response)
    ,

    Status500
    (models::Common500Response)
}


/// API
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Api {

                /// AuthLoginPost - POST /auth/login
                async fn auth_login_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                        body: models::AuthLoginPostRequest,
                ) -> Result<AuthLoginPostResponse, String>;


                /// AuthLogoutPost - POST /auth/logout
                async fn auth_logout_post(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                ) -> Result<AuthLogoutPostResponse, String>;


                /// AuthMeGet - GET /auth/me
                async fn auth_me_get(
                &self,
                method: Method,
                host: Host,
                cookies: CookieJar,
                ) -> Result<AuthMeGetResponse, String>;

}

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod types;

#[cfg(feature = "server")]
pub(crate) mod header;
