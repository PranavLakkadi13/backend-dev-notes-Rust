## GraphQL
GraphQL is a query language and runtime for APIs that lets clients request exactly the data they need from a server, reducing over-fetching/under-fetching common in REST.  <br>
It uses a schema to define types and operations (queries, mutations, subscriptions), enabling flexible, efficient data retrieval.  <br>



# example 

the GraphQL endpoint is available at:
```
http://localhost:30000/gql
```
```graphql
query GetUserById {
    query GetUserById {
    getUserById(id: "3") {
        phone
        address
        city
        organtzation
        email
        name
        id
    }
}
}
```
The Above is the generic example structure of graphql <br>
the query as u can see the required field is just id, and when retreiving the data it can choose what all fields it needs itstead of fetching all the data like in REST API.


The below example just returns the highlighted fields 
```graphql
query GetUserById {
    query GetUserById {
    getUserById(id: "3") {
        phone
        address
        city
    }
}
}

