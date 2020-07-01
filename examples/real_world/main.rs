use epic::{Epic};

fn main() {
  let mut app = Epic::new();

  app.use(UserController.router);
}
