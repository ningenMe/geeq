use std::collections::HashMap;

use axum::{body::Body, extract::*, response::Response, routing::*};
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::{header::CONTENT_TYPE, HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use tracing::error;
use validator::{Validate, ValidationErrors};

use crate::{header, types::*};

#[allow(unused_imports)]
use crate::models;

use crate::{Api,
     AuthOauthPostResponse
};

/// Setup API Server.
pub fn new<I, A>(api_impl: I) -> Router
where
    I: AsRef<A> + Clone + Send + Sync + 'static,
    A: Api + 'static,
{
    // build our application with a route
    Router::new()
        .route("/auth/oauth",
            post(auth_oauth_post::<I, A>)
        )
        .with_state(api_impl)
}

    #[derive(validator::Validate)]
    #[allow(dead_code)]
    struct AuthOauthPostBodyValidator<'a> {
            #[validate]
          body: &'a models::OauthPostRequestBody,
    }


#[tracing::instrument(skip_all)]
fn auth_oauth_post_validation(
        body: models::OauthPostRequestBody,
) -> std::result::Result<(
        models::OauthPostRequestBody,
), ValidationErrors>
{
              let b = AuthOauthPostBodyValidator { body: &body };
              b.validate()?;

Ok((
    body,
))
}

/// AuthOauthPost - POST /auth/oauth
#[tracing::instrument(skip_all)]
async fn auth_oauth_post<I, A>(
  method: Method,
  host: Host,
  cookies: CookieJar,
 State(api_impl): State<I>,
          Json(body): Json<models::OauthPostRequestBody>,
) -> Result<Response, StatusCode>
where 
    I: AsRef<A> + Send + Sync,
    A: Api,
{

      #[allow(clippy::redundant_closure)]
      let validation = tokio::task::spawn_blocking(move || 
    auth_oauth_post_validation(
          body,
    )
  ).await.unwrap();

  let Ok((
      body,
  )) = validation else {
    return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(validation.unwrap_err().to_string()))
            .map_err(|_| StatusCode::BAD_REQUEST); 
  };

  let result = api_impl.as_ref().auth_oauth_post(
      method,
      host,
      cookies,
              body,
  ).await;

  let mut response = Response::builder();

  let resp = match result {
                                            Ok(rsp) => match rsp {
                                                AuthOauthPostResponse::Status200
                                                    {
                                                        body,
                                                        set_cookie
                                                    }
                                                => {
                                                    if let Some(set_cookie) = set_cookie {
                                                    let set_cookie = match header::IntoHeaderValue(set_cookie).try_into() {
                                                        Ok(val) => val,
                                                        Err(e) => {
                                                            return Response::builder()
                                                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                                                    .body(Body::from(format!("An internal server error occurred handling set_cookie header - {}", e))).map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR });
                                                        }
                                                    };

                                                    
                                                    {
                                                      let mut response_headers = response.headers_mut().unwrap();
                                                      response_headers.insert(
                                                          HeaderName::from_static("set-cookie"),
                                                          set_cookie
                                                      );
                                                    }
                                                    }

                                                  let mut response = response.status(200);
                                                  {
                                                    let mut response_headers = response.headers_mut().unwrap();
                                                    response_headers.insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json").map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })?);
                                                  }

                                                  let body_content =  tokio::task::spawn_blocking(move ||
                                                      serde_json::to_vec(&body).map_err(|e| {
                                                        error!(error = ?e);
                                                        StatusCode::INTERNAL_SERVER_ERROR
                                                      })).await.unwrap()?;
                                                  response.body(Body::from(body_content))
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                response.status(500).body(Body::empty())
                                            },
                                        };

                                        resp.map_err(|e| { error!(error = ?e); StatusCode::INTERNAL_SERVER_ERROR })
}

