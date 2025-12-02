use async_graphql::{ID, Object};

#[derive(Clone)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub city: String,
    pub organtzation: String,
}

// this make the struct as a graph ql object and allows it to access the fields in async way
// making the model queryable
#[Object]
impl User {
    // this is the model when we are woking with graph ql
    async fn id(&self) -> &str {
        &self.id.0
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn phone(&self) -> &str {
        &self.phone
    }

    async fn address(&self) -> &str {
        &self.address
    }

    async fn city(&self) -> &str {
        &self.city
    }

    async fn organtzation(&self) -> &str {
        &self.organtzation
    }
}
