# Epic

Fast, minimalist web framework for Rust.

Epic is an Express like framework for Rust. It focuses on speed and simplicity.

```Rust
fn main() {
  let mut app = Epic::new();

  app.use("/", (req: &mut Request, res: &mut Response) -> MiddlewareResult {
    res.send("Hello, World!");
  });

  app.listen("127.0.0.1:7878").unwrap();
}
```
