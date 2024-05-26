pub struct User {
    user_id: String,
    avatar_url: String,
}

impl User {
    pub fn new(user_id: String, avatar_url: String) -> User {
        //TODO バリデーション
        User {
            user_id: user_id,
            avatar_url: avatar_url,
        }
    }
    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
}
