extern crate tyger;

use tyger::Response;

fn main() {
    tyger::serve(|_| Ok(Response::new("Hello, world!".into()))).unwrap();
}
