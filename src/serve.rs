use std::io;
use std::rc::Rc;

use bytes::{Bytes, BytesMut};
use configure::Configure;
use failure::Error;
use futures::{Future, IntoFuture, Stream, stream};
use http::{Request, Response};
use hyper;
use hyper::server::{Http, service_fn};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

use Config;

pub fn serve<F, Fut>(app: F) -> Result<(), Error> where
    F: Fn(Request<Bytes>) -> Fut + 'static,
    Fut: IntoFuture<Item = Response<Bytes>, Error = ()>,
{
    let cfg = Config::generate()?;
    let mut core = Core::new()?;
    let handle = core.handle();
    let listener = TcpListener::bind(&cfg.addr, &handle)?;
    let http = Http::new();
    let app = Rc::new(app);

    let serve = listener.incoming().for_each(move |(socket, remote_addr)| {
        let app = app.clone();

        let hyper_shim = service_fn(move |req: Request<hyper::Body>| {
            let app = app.clone();
            let (parts, body) = req.into_parts();

            let req = body.fold(BytesMut::new(), |mut body, chunk| -> Result<_, hyper::Error> {
                body.extend(chunk);
                Ok(body)
            }).map(move |body| Request::from_parts(parts, body.into()));
            
            req.and_then(move |req| app(req).into_future().then(|result| match result {
                Ok(resp)    => {
                    let (parts, body) = resp.into_parts();
                    Ok(Response::from_parts(parts, stream::once(Ok(body))))
                }
                Err(_)      => {
                    Err(io::Error::new(io::ErrorKind::Other, "").into())
                }
            }))
        });

        http.bind_connection_compat(&handle, socket, remote_addr, hyper_shim);
        Ok(())
    });

    core.run(serve)?;

    Ok(())
}
