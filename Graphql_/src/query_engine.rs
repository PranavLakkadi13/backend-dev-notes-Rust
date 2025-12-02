use crate::{db::DB, user_service::User};
use async_graphql::Object;

pub struct Query {
    pub db: DB,
}

// here what we did is allowing the Query to access the Db struct in state (like we pass actual db in axum)
#[Object]
impl Query {
    async fn get_user_by_id(&self, id: String) -> Option<User> {
        self.db.get_data().iter().find(|&x| x.id.0 == id).cloned()
    }

    async fn get_users(&self) -> Vec<User> {
        self.db.get_data()
    }
}
