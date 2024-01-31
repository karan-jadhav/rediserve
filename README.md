# RediServe: HTTP API for Redis

## Introduction

RediServe is an innovative, open-source tool providing HTTP API access for Redis operations. This Rust-based project bridges the gap between Redis and HTTP, enabling users to interact with Redis using straightforward HTTP requests. One of its significant advantages is full compatibility with the Upstash SDK, making it a valuable tool for developers familiar with Upstash's Redis REST API.

## ⚠️ Development Status

RediServe is currently in the early stages of development. It is not yet ready for production use. We are actively working on the project and will be releasing a stable version soon.

## Usage

Todo

## Installation

Todo

## API Endpoint Status

- [x] Basic Redis commands API
- [ ] Transactions API
- [ ] Pipelining API
- [ ] Authentication API
- [ ] Support for Redis Cluster


## Contributing

We welcome contributions from the community. To contribute

1. Fork the Project
2. Create your Feature Branch (git checkout -b feature/YourFeature)
3. Commit your Changes (git commit -m 'Add some YourFeature')
4. Push to the Branch (git push origin feature/YourFeature)
5. Open a Pull Request

## License

RediServe is licensed under the [Apache License, Version 2.0](LICENSE).


## Acknowledgements

**Inspiration**: A heartfelt thank you to the Upstash Redis REST API, whose implementation has been a significant source of inspiration for RediServe.

### Libraries and Tools:

**axum**: For its efficient and robust web application framework capabilities in Rust.

**deadpool-redis**: For enhancing RediServe’s performance with effective Redis connection pooling.

**redis-rs**: Providing the comprehensive Rust client for Redis, integral to RediServe's functionality.

**tokio**: For its support in asynchronous Rust programming, enabling efficient scaling and performance.

### Personal Note

As the current sole developer, this project represents a journey of learning and growth. The experiences and challenges faced have been invaluable for personal and professional development.

The future contributions from the community and continued support from the open-source ecosystem are eagerly anticipated as RediServe continues to evolve.