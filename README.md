http_test_server
=========

A tiny HTTP server for testing and debugging. It prints incoming HTTP requests
(the request line, headers, and body) to the terminal so you can inspect what
clients send during development. Could also be used as a starting template for a hyper project.

Quick start
-----------

- Build:

  `cargo build --release`

- Run:

  `cargo run --release`
  or
  `cargo run --release -- -p 8000`

- Send a test request (from another terminal):

  `curl -v http://127.0.0.1:3000/hello -H "X-Test: example" -d "payload"`

What it does
------------

- Listens for HTTP requests on the configured port.
- Logs the request method, path, headers, and body to stdout.
- Useful for debugging webhooks, proxies, and clients during development.

Implementation
--------------

See the server entrypoint: [src/main.rs](src/main.rs)

License
-------

MIT
