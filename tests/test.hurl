# Test happy path
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/test",
        "method": "POST",
        "query": "param1=value1&param2=value2",
        "headers": {
            "content-type": "application/json"
        }
    },
    "response": {
        "code": 201,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json",
            "x-custom-header": "custom-value"
        }
    }
}

HTTP 201

POST http://127.0.0.1:3000/test?param1=value1&param2=value2
Content-Type: application/json
{
    "name": "Test request"
}

HTTP 201
[Asserts]
header "Content-Type" == "application/json"
header "X-Custom-Header" == "custom-value"
body == "{\"name\":\"Test response\"}"

GET http://127.0.0.1:3000/unknown-test
HTTP 404

# Test with no body
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/test-with-no-body",
        "method": "POST"
    },
    "response": {
        "code": 204,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

POST http://127.0.0.1:3000/test-with-no-body

HTTP 204
[Asserts]
header "Content-Type" == "application/json"
body == "" # FIXME: This should be {\"name\":\"Test response\"}

# Test method get
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/test-method-get",
        "method": "GET"
    },
    "response": {
        "code": 200,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

GET http://127.0.0.1:3000/test-method-get

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
body == "{\"name\":\"Test response\"}"

# Test method delete
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/test-method-delete",
        "method": "DELETE"
    },
    "response": {
        "code": 200,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

DELETE http://127.0.0.1:3000/test-method-delete

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
body == "{\"name\":\"Test response\"}"

# Test method patch
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/test-method-patch",
        "method": "PATCH"
    },
    "response": {
        "code": 200,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

PATCH http://127.0.0.1:3000/test-method-patch

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
body == "{\"name\":\"Test response\"}"

# Test method put
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/test-method-put",
        "method": "PUT"
    },
    "response": {
        "code": 200,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

PUT http://127.0.0.1:3000/test-method-put

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
body == "{\"name\":\"Test response\"}"

# Test empty path
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/",
        "method": "GET"
    },
    "response": {
        "code": 200,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

GET http://127.0.0.1:3000

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
body == "{\"name\":\"Test response\"}"

# Test invalid headers
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/",
        "method": "GET",
        "headers": {
            "X-Request-Id": "1"
        }
    },
    "response": {
        "code": 200,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

GET http://127.0.0.1:3000
X-Request-Id: 2

HTTP 500

# Test delete a configuration
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/will-be-deleted",
        "method": "GET"
    },
    "response": {
        "code": 200,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

DELETE http://127.0.0.1:3001/configure
{
    "request": {
        "path": "/will-be-deleted",
        "method": "GET"
    }
}

HTTP 204

GET http://127.0.0.1:3000/will-be-deleted

HTTP 404

# Test clear all configurations
POST http://127.0.0.1:3001/configure
{
    "name": "Test request",
    "request": {
        "path": "/will-be-deleted",
        "method": "GET"
    },
    "response": {
        "code": 200,
        "body": "{\"name\":\"Test response\"}",
        "headers": {
            "content-type": "application/json"
        }
    }
}

HTTP 201

POST http://127.0.0.1:3001/clear

HTTP 204

GET http://127.0.0.1:3000/will-be-deleted

HTTP 404
