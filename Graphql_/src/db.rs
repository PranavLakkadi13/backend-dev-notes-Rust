use crate::user_service::User;
use async_graphql::ID;

pub struct DB;

// using a array as a db for the moment
impl DB {
    pub fn get_data(&self) -> Vec<User> {
        vec![
            User {
                id: ID::from("1"),
                name: "Alice".to_string(),
                email: "Alice@xyz.com".to_string(),
                phone: "99223344113".to_string(),
                address: "James street".to_string(),
                city: "New york".to_string(),
                organtzation: "TechCom".to_string(),
            },
            User {
                id: ID::from("2"),
                name: "Bob".to_string(),
                email: "Bob@abc.com".to_string(),
                phone: "1234567890".to_string(),
                address: "Oak Avenue".to_string(),
                city: "Los Angeles".to_string(),
                organtzation: "InnovateInc".to_string(),
            },
            User {
                id: ID::from("3"),
                name: "Charlie".to_string(),
                email: "Charlie@def.com".to_string(),
                phone: "9876543210".to_string(),
                address: "Pine Road".to_string(),
                city: "Chicago".to_string(),
                organtzation: "DataCorp".to_string(),
            },
            User {
                id: ID::from("4"),
                name: "Diana".to_string(),
                email: "Diana@ghi.com".to_string(),
                phone: "5556667777".to_string(),
                address: "Elm Street".to_string(),
                city: "Houston".to_string(),
                organtzation: "FutureTech".to_string(),
            },
            User {
                id: ID::from("5"),
                name: "Eve".to_string(),
                email: "Eve@jkl.com".to_string(),
                phone: "1112223333".to_string(),
                address: "Maple Lane".to_string(),
                city: "Phoenix".to_string(),
                organtzation: "GlobalSys".to_string(),
            },
        ]
    }
}
