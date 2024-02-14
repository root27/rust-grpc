# Rust Grpc Server 
 
This repo contains grpc server and client using Rust and actix-web. It includes basic CRUD operations with MongoDB.

## How to run the server
```bash

cargo run --bin server

```

## How to run the client
```bash

cargo run --bin client

```

Client includes several APIs to communicate with grpc server.

### APIs

1. Create User

- Request

```json
{
  "name": "John Doe",
  "email": "test@test.com",
  "password": "password",
  "age": 25
}

```

- Response

```json
{
    "message": "User created"
}

```

2. Get User

- Request

```json
{
  "email": ""
}

```

- Response

```json
{
    "name": "",
    "email": "",
    "age": ,
    "message": "User found"
}

```

3. Update User

- Request

```json
{
  "name": "",
  "email": "",
  "age":
}

```

- Response

```json
{
    "message": "User updated successfully"
}

```

4. Delete User

- Request

```json
{
  "email": ""
}

```

- Response

```json
{
    "message": "User deleted"
}

```

