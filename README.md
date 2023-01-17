# RecipesApp
Peer 2 peer recipes app written in Rust

# Example
In this example, we will start two clients, "john" and "jane", and a server. The clients will connect to the server and exchange messages using the floodsub protocol.

1. Start on one ubuntu client by running cargo run -- john

2. Start on secobd ubuntu client by running cargo run -- jane

3. These clients have their own recipe database, WIP for adding new clients

4. Commands that are supported are "ls p" for listing connected peers and "ls r" for listing recipes