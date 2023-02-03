### Bidirectional gRPC Streaming with Rust and Tonic

This is a simple example of a bidirectional gRPC streaming service implemented in Rust using the [Tonic](https://github.com/hyperium/tonic) library.

The service consists of a single RPC method `Multiply` which takes a stream of `CalculatorRequest` messages (single int value) and returns a stream of `CalculatorResponse` messages (inputs value multiply by 2).

What is bidirectional streaming in gRPC?


#### How to run

To run the server:

```bash
cargo run --bin server
```

To run the client:

```bash
cargo run --bin client
```
