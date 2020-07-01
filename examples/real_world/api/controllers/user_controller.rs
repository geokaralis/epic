use epic::{Router, Request, Response, MiddlewareResult};

pub struct UserController {
  router: Router,
}

impl UserController {
  pub fn find_one(req: &mut Request, res: &mut Response) {
    let user_email = req.params.email
    let user = UserService::find_one(user_email);

    res.send(user)
  }
}
