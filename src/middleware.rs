use hyper::net::Fresh;
use request::Request;
use response::Response;

pub trait Middleware: Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: Request<'m, 'r>, res: Response<'m, Fresh>);
}

impl<T> Middleware for T where T: for <'m, 'r> Fn(Request<'m, 'r>, Response<'m>) + Send + Sync + 'static {
    fn execute<'m, 'r>(&'m self, req: Request<'m, 'r>, res: Response<'m>) {
        (*self)(req, res);
    }
}