use redis::Commands;
use rocket::{
    Data, Request,
    fairing::{Fairing, Info, Kind},
    http::uri::Origin,
};

const MAX_REQUEST: i32 = 3;

pub struct RateLimiter {
    pub client: redis::Client,
}

#[rocket::async_trait]
// in rocet Fairings help u create middleware
impl Fairing for RateLimiter {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Custom RateLimiter",
            // If u want to have rate limit in response u can keep Kind::Response
            kind: Kind::Request, // This is because we just want to rate limit the requests
        }
    }

    async fn on_request(&self, _req: &mut Request<'_>, _data: &mut Data<'_>) {
        // use req.real_ip() when the server is hosted behind a load balancer or proxy
        let ip = _req.client_ip().unwrap().to_string();
        let mut con = self.client.get_connection().unwrap();
        let count: Result<i32, redis::RedisError> = con.get(&ip);

        if let Ok(count) = count {
            if count > MAX_REQUEST {
                // send error
                // when the limit is hit it will be rerouted to this endpoint
                _req.set_uri(Origin::parse("/429").unwrap());
            } else {
                let _: () = con.incr(&ip, 1).unwrap();
            }
        } else {
            // if the user is here for the first time then we will have time to live in cache with the max being 3 requests in 60 sec
            // i could use con.expire() to set the time of expiry but the set_ex checks if its first time 
            let _: () = con.set_ex(&ip, 1, 60).unwrap(); 
        }
    }
}
