# RecipesApp
Peer 2 peer recipes app written in Rust

# Example
In this example, we will start two clients, "john" and "jane", and a server. The clients will connect to the server and exchange messages using the floodsub protocol.

### Start on one ubuntu client
```shell
cargo run -- john
```

### Start on secobd ubuntu client 
```shell
cargo run -- jane
```

These clients have their own recipe database, WIP for adding new clients.

### Supported commands
```shell 
ls p
```
for listing connected peers 

```shell
ls r
```
for listing recipes