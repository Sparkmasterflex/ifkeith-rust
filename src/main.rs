extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
  let mut router = Router::new();

  router.get("/", suckit_nerds, "index");
  router.get("/:query", suckit_nerds, "show");

  fn suckit_nerds(req: &mut Request) -> IronResult<Response> {
    let ref query = match req.extensions.get::<Router>().unwrap().find("query") {
      Some(q) => q,
      None => "Nerds"
    };

    Ok( Response::with(
      (status::Ok, format!("Suckit {}!", query))
    ) )
  }


  Iron::new(router).http("localhost:3000").unwrap();
  println!("On 3000");
}
