# API Documentation

This document describes the HTTP API provided by the URL Shortener.

## Endpoints

### Create a shortened URL

**POST** `/urls`

Creates a shortened URL from the URL provided in the request body.

#### Request

```
http
POST /urls HTTP/1.1
Content-Type: application/json

{
  "original_url": "https://example.com"
}
```

#### Response

```
HTTP/1.1 201 Created
Content-Type: application/json

{
  "message": "Short URL created successfully",
  "status": "success",
  "token": "X14dav-exemple"
}
```

### Redirect to the original URL

**GET** `/{token}`

Redirects the client to the original URL associated with the token.

#### Request

```
http
GET /X14dav-exemple HTTP/1.1
```

#### Response

```
HTTP/1.1 307 Temporary Redirect
location: https://example.com
```

### Get URL information

**GET** `/urls/<token>`

Returns information about the URL associated with the given token.

#### Request

```
http
GET /urls/X14dav-exemple HTTP/1.1
```

#### Response

```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "created_at": "2026-08-03T18:24:11.953980Z",
  "message": "Short URL info retrieved successfully",
  "original_url": "https://example.com",
  "status": "success",
  "token": "X14dav-exemple"
}
```

### Delete a shortened URL

**DELETE** `/urls/<token>`

Deletes the shortened URL associated with the given token.

#### Request

```
http
DELETE /urls/X14dav-exemple HTTP/1.1
```

#### Response

```
HTTP/1.1 200 OK
Content-Type: application/json

{
  "message": "Short URL deleted successfully",
  "status": "success"
}
```
