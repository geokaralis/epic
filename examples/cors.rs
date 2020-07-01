use epic::{Epic, Request, Response, MiddlewareResult};
use epic::cors;

fn main() {
  let mut app = Epic::new();
  // Enable cors middleware
  app.use(cors);

  app.use("/", (_req: &mut Request, res: &mut Response) -> MiddlewareResult {
    res.send("Hello, Cors!");
  });

  app.listen("127.0.0.1:7878").unwrap();
}
