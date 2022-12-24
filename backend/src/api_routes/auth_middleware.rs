use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage,
};
use futures::FutureExt;
use futures_util::future::LocalBoxFuture;

use crate::{
    app::app_helpers::app_request_helper::pluck_token_and_role,
    user::user_services::user_service::UserService,
};

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Authenticating path: {}", req.path());

        let user_service = req
            .app_data::<web::Data<UserService>>()
            .expect("could not get user service")
            .clone();

        let srv = self.service.clone();

        async move {
            let token_and_role = pluck_token_and_role(&req);
            let mut is_authenticated = false;
            if let Some(token) = token_and_role.0 {
                let user = user_service
                    .as_ref()
                    .authenticate_by_token(token.as_str())
                    .await;

                if let Some(auth_user) = user {
                    req.extensions_mut().insert(auth_user);
                    is_authenticated = true
                }
            }

            // @todo handle when authentication fails
            // Not sure how I can return the same typ as `fut.await?` below

            // if !is_authenticated {
            //     println!("Authentication failed");
            //     let t = HttpResponse::Ok().body("You rare not authenticated");
            //     let response = HttpResponse::Unauthorized()
            //         .body("not authenticated");
            //     let r : ServiceResponse<B> = ServiceResponse::new(req.request().clone(), response.try_into().unwrap());
            //     Ok(r)
            // } else {
            // }

            println!("Authentication passed: {}", is_authenticated);
            let fut = srv.call(req);
            let res = fut.await?;
            Ok(res)
        }
        .boxed_local()
    }
}
