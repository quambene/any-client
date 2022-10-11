<!-- markdownlint-disable MD024 -->

# any-client

Utility tool for testing http, websocket, and grpc endpoints.

- [Install any-client](#install-any-client)
- [Usage](#usage)
- [Development](#development)
- [Configure HTTP client](#configure-http-client)
- [Configure websocket client](#configure-websocket-client)
- [Configure grpc client](#configure-grpc-client)

## Install any-client

``` bash
git clone git@github.com:quambene/any-client.git
cd any-client
cargo install --path .
```

## Usage

``` bash
CONFIG_PATH=config.json any-client
```

## Development

``` bash
cd any-client # path to any-client
CONFIG_PATH=config.json cargo run
```

## Configure HTTP client

Create config file, e.g. `config.json`, with content:

``` json
{
    "client": "http",
    "api": {
        "url": "https://...",
        "endpoint": "/my_endpoint"
    },
    "request": {
        "method": "POST",
        "headers": [{"key": "MY_KEY", "value": "my_value"}],
        "body": {
            "myKey": "my_value"
        },
        "queryString": {
            "myKey": "my_value"
        }
    }
}
```

## Configure websocket client

Create config file, e.g. `config.json`, with content:

``` json
{
    "client": "websocket",
    "api": {
        "url": "wss://my_url.com",
        "endpoint": "/my_endpoint"
    },
    "subscription": {
        "query_string": {
            "myParam1": "my_value1",
            "myParam2": "my_value2"
        },
        "request": {
            "myKey": "my_value"
        }
    }
}
```

where `query_string` and `request` are optional. The `query_string` is leading to the url `wss://my_url.com/my_endoint?myParam1=my_value1&myParam2=my_value2`.

## Configure grpc client

Create config file, e.g. `config.json`, with content:

``` json
{
    "client": "grpc",
    "api": {
        "url": "https://...",
    },
    "proto": {
        "path": "my_proto_dir",
        "file": "my_proto_file.proto",
        "package": "my_package",
        "service": "MyService",
        "method": "MyMethod",
        "message": "MyMessage",
        "request": {
            "myKey": "my_value"
        }
    }
}
```
