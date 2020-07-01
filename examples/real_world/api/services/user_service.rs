pub struct UserService {}

impl UserService {
  pub fn find_one(email: String) {
    let user = UserRepository.find_one(email);

    if (!user) return "User not found";

    return user;
  }
}
