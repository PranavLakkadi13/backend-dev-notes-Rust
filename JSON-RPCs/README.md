## JSON RPCs
JSON-RPC is a lightweight, stateless remote procedure call (RPC) protocol that uses JSON for encoding requests and responses. It is transport-agnostic, meaning it can be used over various transport protocols such as HTTP, WebSocket, or TCP.

## What RPC Means:
Remote Procedure Call = Calling a function on a different machine/server as if it were local.

-> How It Works:  
You (Client) → Send a request to a Server asking it to run a specific function.  
Server → Executes that function with the parameters you provided.  
Server → Sends back the result. 


# example used in postman 

with the `POST` request to ``http://localhost:3000/``
```JSON
{
    "jsonrpc": "2.0",
    "method": "say_hello",
    "id": 2
}
```
```JSON
{
    "jsonrpc": "2.0",
    "method": "add",
    "params": [123,4294967295],
    "id": 2
}
``` 
 -> Response
```JSON
{
    "jsonrpc": "2.0",
    "result": 429496852,
    "id": 2
}   
```


# example of JSON-RPC request 
```bash
curl -X POST 'https://mainnet.infura.io/v3/<YOUR-API-KEY>' \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'   
```