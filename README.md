# URL Shortener

> **Educational project — not intended for production use.**

A simple URL shortener REST API built with Rust, Axum, SQLx, and PostgreSQL.

This project was created **for learning and demonstration purposes.** It serves as a practical exercise for exploring backend development, REST API design, SQL, PostgreSQL, asynchronous Rust, database migrations, connection pooling, and Docker.

## What I'm learning

This project is being used to experiment with and improve my understanding of:

- REST API design
- PostgreSQL and SQL
- Database migrations
- Docker and containerized development
- HTTP methods and status codes
- Programming with Rust
- Async programming in Rust
- Caching strategies and caching in backend services

And perhaps I'll expand this list as I continue to improve this project.

## What this does

This project provides a simple URL shortening service through HTTP requests.

It allows users to submit a URL and receive a shorter URL containing a unique identifier. When the shortened URL is accessed, the service looks up the corresponding original URL and redirects the user to it.

The project is intentionally kept simple and is mainly used to experiment with backend concepts and explore different approaches as the implementation evolves.

## Documentation

See the [API documentation](API.md) for details about the available endpoints.

## Running locally

### Note

Eventually, I will migrate the project to run entirely within a Docker container.
These instructions show how to start the program in the project's current state. Once I migrate to Docker, the process will be more easy and automated.

### Prerequisites

Make sure you have:

- Rust and Cargo
- sqlx-cli
- Docker
- Docker Compose

** 1. Clone the repository **

```
git clone https://github.com/bk304/url-shortener-rs.git && cd url-shortener-rs
```

** 2. Configure environment variables **

Create a .env file based on .env.example:

```
cp .env.example .env
```

Configure the PostgreSQL credentials and database URL in the file.

** 3. Start PostgreSQL **

```
docker compose up -d
```

** 4. Run the migrations **

```
sqlx migrate run
```

** 5. Start the application **

```
cargo run
```

The API will be available at:

http://localhost:3000


