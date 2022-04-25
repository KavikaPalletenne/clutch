# Resource Service

## Environment Variables
```
DATABASE_URL - MongoDB connection URL
ACTIX_PORT - Port HTTP server runs on (inside Docker)
GRPC_PORT - Port gRPC server runs on (inside Docker)
JWT_SECRET - Secret for signing JWTs

# Meilisearch
SEARCH_ENDPOINT
SEARCH_API_KEY

# Object Storage
S3_ACCESS_KEY
S3_SECRET_KEY
S3_ENDPOINT
S3_REGION_NAME
```

## Endpoints

### HTTP

`POST /api/resource/create`

Request:
```json
{
    "group_id": "65432839356685232649",
    "title": "Edrolo Biology 3/4 Textbook",
    "description": "Full textbook",
    "subject": "Physics",
    "tags": [
        "Biology",
        "Textbook"
    ],
    "files": [
        {
            "name": "edrolo_biology_34.pdf",
            "size":  33554432 // Num. bytes
        }
    ]
}
```

Response:
```json
{
    "resource_id": "9_HRl2XjSjAU4lbnNP-LG",
    "group_id" : "65432839356685232649",
    "file_put_urls" : [
        "http://api.examclutch.com:8080/excl/65432839356685232649/9_HRl2XjSjAU4lbnNP-LG/Edorolo%20VCE%20Biology%20Units%203%20%26%204%20Textbook.pdf?X-Amz-..."
    ],
}
```

`GET /api/resource/get/{id}`
```json
{
    "_id": "9_HRl2XjSjAU4lbnNP-LG",
    "user_id": "436035620905943041",
    "group_id": "65432839356685232649",
    "title": "Edrolo Biology 3/4 Textbook",
    "description":  "Full textbook",
    "subject": "Biology",
    "tags": [
        "Biology",
        "Textbook"
    ],
    "files": [
        {
            "name": "edrolo_biology_34.pdf",
            "size":  33554432
        }
    ],
    "last_edited_at": "2022-04-21T04:04:34.944551900"
}
```

`GET /api/resource/get_all/{group_id}`
```json
{ [
    {
        "_id": "QNVRH1Uf_3Yr-XpePM15m",
        "user_id": "436035620905943041",
        "group_id": "65432839356685232649",
        "title": "2017 VCE Argument Analysis Essay",
        "description": "",
        "subject": "English",
        "tags": [
            "English",
            "Argument Analysis"
            ],
        "files": [
            {
                "name": "2017 VCE Argument Analysis.pdf",
                "size": 588179
            }
        ],
        "last_edited_at":"2022-04-21T04:04:34.944551900"
    },
    {
        "_id": "9_HRl2XjSjAU4lbnNP-LG",
        "user_id": "436035620905943041",
        "group_id": "65432839356685232649",
        "title": "Edrolo Biology 3/4 Textbook",
        "description":  "Full textbook",
        "subject": "Biology",
        "tags": [
            "Biology",
            "Textbook"
        ],
        "files": [
            {
                "name": "edrolo_biology_34.pdf",
                "size":  33554432
            }
        ],
        "last_edited_at": "2022-04-21T04:04:34.944551900"
    }
]
}
```

`GET /api/resource/getByUserId/{user_id}`
```json
{ [
    {
        "_id": "QNVRH1Uf_3Yr-XpePM15m",
        "user_id": "436035620905943041",
        "group_id": "65432839356685232649",
        "title": "2017 VCE Argument Analysis Essay",
        "description": "",
        "subject": "English",
        "tags": [
            "English",
            "Argument Analysis"
            ],
        "files": [
            {
                "name": "2017 VCE Argument Analysis.pdf",
                "size": 588179
            }
        ],
        "last_edited_at":"2022-04-21T04:04:34.944551900"
    },
    {
        "_id": "9_HRl2XjSjAU4lbnNP-LG",
        "user_id": "436035620905943041",
        "group_id": "65432839356685232649",
        "title": "Edrolo Biology 3/4 Textbook",
        "description":  "Full textbook",
        "subject": "Biology",
        "tags": [
            "Biology",
            "Textbook"
        ],
        "files": [
            {
                "name": "edrolo_biology_34.pdf",
                "size":  33554432
            }
        ],
        "last_edited_at": "2022-04-21T04:04:34.944551900"
    }
]
}
```

`POST /api/resource/update/{id}`

Request:
```json
{
    "group_id": "65432839356685232649",
    "title": "Edrolo Biology 3/4 Textbook",
    "description": "Full textbook",
    "subject": "Physics",
    "tags": [
        "Biology",
        "Textbook"
    ],
    "files": [
        {
            "name": "edrolo_biology_34.pdf",
            "size":  33554432 // Num. bytes
        }
    ]
}
```

Response:
```text
200 Ok
or
400 Bad Request
```

`GET /api/resource/delete/{id}`
```text
200 Ok
or
400 Bad Request
or
401 Unauthorized
```