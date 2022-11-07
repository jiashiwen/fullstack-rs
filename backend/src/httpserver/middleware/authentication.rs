use crate::privilege::get_user_id_from_token;
use axum::http::{header, Request, Response, StatusCode};
use http_body::Body;
use std::marker::PhantomData;
use tower_http::auth::AuthorizeRequest;

pub struct MyAuth<ResBody> {
    pub(crate) _ty: PhantomData<fn() -> ResBody>,
}

impl<ResBody> Clone for MyAuth<ResBody> {
    fn clone(&self) -> Self {
        Self { _ty: PhantomData }
    }
}

impl<B, ResBody> AuthorizeRequest<B> for MyAuth<ResBody>
where
    ResBody: Body + Default,
{
    type ResponseBody = ResBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let i = request.headers().get(header::AUTHORIZATION);
        return match i {
            None => {
                let body = ResBody::default();
                let mut res = Response::new(body);
                *res.status_mut() = StatusCode::UNAUTHORIZED;
                Err(res)
            }
            Some(hv) => {
                let hstr = hv.to_str();

                match hstr {
                    Ok(str) => {
                        let token_exists = get_user_id_from_token(str.to_string());

                        if let Err(_) = token_exists {
                            let body = ResBody::default();
                            let mut res = Response::new(body);
                            *res.status_mut() = StatusCode::UNAUTHORIZED;
                            return Err(res);
                        } else {
                            Ok(())
                        }
                    }
                    Err(_) => {
                        let body = ResBody::default();
                        let mut res = Response::new(body);
                        *res.status_mut() = StatusCode::UNAUTHORIZED;
                        Err(res)
                    }
                }
            }
        };
    }
}
