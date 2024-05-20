# Mockser

[![Crates.io](https://img.shields.io/crates/v/mockser.svg)](https://crates.io/crates/mockser)

Configurable mock server for testing and development.

The repository contains the implementation of a mock server that allows configuring and handling mock requests.
The server listens on a specified host and port, and provides an HTTP endpoint for configuring mocks.

The server uses the Axum framework for handling HTTP requests and responses.
It maintains a state that holds the configurations of the mocks, which are stored in a HashMap.

The server also listens on a separate port for configuration requests, allowing dynamic updates to the mock configurations.

To configure a mock, send a POST request to the `/configure` endpoint with the mock configuration in the request body.
The mock configuration should include the request URI, method, body, and headers, as well as the response code, body, and headers.

The server matches incoming requests with the configured mocks based on the request path.
If a matching mock is found, it constructs an HTTP response using the mock's response code, body, and headers.
If no matching mock is found, it returns a 404 Not Found response.

You can see the server in action by running the tests by `hurl tests/test.hurl`.

## Installation

```sh
cargo install mockser
```

[Releases](https://github.com/daxartio/mockser/releases)

```
docker run --rm -p 3000:3000 -p 3001:3001 daxart/mockser
```

## Usage

```
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
```

## Configuration

The server can be configured using environment variables.

| name                    | type   | default   | options             |
|-------------------------|--------|-----------|---------------------|
| MOCKSER_HOST            | string | '0.0.0.0' |                     |
| MOCKSER_PORT            | number | 3000      |                     |
| MOCKSER_CONFIG_PORT     | number | 3001      |                     |
| MOCKSER_INITIAL_CONFIGS | path   | None      |                     |
| MOCKSER_LOG             | string | 'info'    | ...                 |
| MOCKSER_LOG_STYLE       | string | 'auto'    | auto, always, never |
| MOCKSER_LOG_FORMAT      | string | 'human'   | human, json         |
