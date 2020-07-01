use epic::{Epic, Request, Response, MiddlewareResult};

fn main() {
  let mut app = Epic::new();
  
  app.get("/", (_req: &mut Request, res: &mut Response) -> MiddlewareResult {
    res.send("Hello, World!");
  });
  // Listen on port 7878
  app.listen("127.0.0.1:7878").unwrap();
}
