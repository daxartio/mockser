# Mockser

WIP

Configurable mock server for testing and development.

The repository contains the implementation of a mock server that allows configuring and handling mock requests.
The server listens on a specified host and port, and provides an HTTP endpoint for configuring mocks.

The server uses the Axum framework for handling HTTP requests and responses.
It maintains a state that holds the configurations of the mocks, which are stored in a HashMap.

The server also listens on a separate port for configuration requests, allowing dynamic updates to the mock configurations.
It gracefully shuts down when a SIGINT signal is received. SIGTERM is in progress.

To configure a mock, send a POST request to the `/configure` endpoint with the mock configuration in the request body.
The mock configuration should include the request URI, method, body, and headers, as well as the response code, body, and headers.

The server matches incoming requests with the configured mocks based on the request path.
If a matching mock is found, it constructs an HTTP response using the mock's response code, body, and headers.
If no matching mock is found, it returns a 404 Not Found response.

You can see the server in action by running the tests by `hurl tests/test.hurl`.

I will add builds and releases soon.
