use env_logger::Builder;
use log::{Level, info};
use std::io::Write;
use std::future::{ready, Ready};
use actix_web::{
   dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
   Error,
};
use futures_util::future::LocalBoxFuture;

pub struct LoggerMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggerMiddleware
where
   S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
   S::Future: 'static,
   B: 'static,
{
   type Response = ServiceResponse<B>;
   type Error = Error;
   type InitError = ();
   type Transform = LoggerMiddlewareService<S>;
   type Future = Ready<Result<Self::Transform, Self::InitError>>;

   fn new_transform(&self, service: S) -> Self::Future {
      ready(Ok(LoggerMiddlewareService { service }))
   }
}

pub struct LoggerMiddlewareService<S> {
   service: S
}

impl<S, B> Service<ServiceRequest> for LoggerMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
   type Response = ServiceResponse<B>;
   type Error = Error;
   type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

   forward_ready!(service);

   fn call(&self, req: ServiceRequest) -> Self::Future {
      info!("server request: {} {}", req.method(), req.uri());
      let fut = self.service.call(req);

      Box::pin(async move {
         let res = fut.await?;
         info!("server response: {}", res.status());
         Ok(res)
      })
   }
}

pub fn init_logger() {
   Builder::from_default_env()
   .parse_default_env()
   .format(|buf, record| {
      let level = record.level();
      let color_level = match level {
         Level::Error => "\x1b[31;1m", // Красный
         Level::Warn => "\x1b[33;1m", // Желтый
         Level::Info => "\x1b[32;1m", // Зеленый
         Level::Debug => "\x1b[34;1m", // Синий
         Level::Trace => "\x1b[35;1m", // Пурпурный
      };
      writeln!(buf, "{}{} - {}\x1b[0m", color_level, level, record.args())
   })
   .init()
}

