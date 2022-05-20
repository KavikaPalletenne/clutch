# User Service

## Environment Variables
```
DATABASE_URL - MongoDB connection URL
ACTIX_PORT - Port HTTP server runs on (inside Docker)
GRPC_PORT - Port gRPC server runs on (inside Docker)
JWT_SECRET - Secret for signing JWTs
```

## Endpoints

### HTTP

`GET /api/user/{id}`
```json
{
    "_id": "V1StGXR8_Z5jdHi6B-myT",
    "username": "Endveous",
    "email": "me@domain.com",
    "groups": [
        "65432839356685232649",
        "62843245932068783983"
    ]
}
```

`GET /api/user/username/{id}`
```json
{
    "username": "Endveous"
}
```

`POST /api/user/updateUsername/{id}/{username}`
```text
200 Ok
or
400 Bad Request
```

`POST /api/user/updateEmail/{id}/{email}`
```text
200 Ok
or
400 Bad Request
```

`GET /api/user/delete/{id}`
```text
200 Ok
or
400 Bad Request
```

`GET /api/user/get_user_groups/{id}`
```json
{
    [
        "65432839356685232649",
        "62843245932068783983"
    ]
}
```


### gRPC
```proto
service UserService {
    CreateUser (CreateUserRequest) returns (CreateUserResponse)
    UserExists (UserExistsRequest) returns (UserExistsResponse)
}

message CreateUserRequest {
    string id = 1;
    string username = 2;
    string email = 3;
}

message CreateUserResponse {
    bool success = 1;
}

message UserExistsRequest {
    string user_id = 1;
}

message UserExistsResponse {
    bool exists = 1;
}
```