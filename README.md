# TestRoute

[![Crates.io Version](https://img.shields.io/crates/v/testroute)](https://crates.io/crates/testroute)
[![GitHub's license](https://img.shields.io/github/license/cherryramatisdev/testroute)](https://github.com/cherryramatisdev/testroute/blob/main/LICENSE)
[![GitHub last commit](https://img.shields.io/github/last-commit/cherryramatisdev/testroute/main)](https://github.com/cherryramatisdev/testroute)

> A project to create test http routes for your closest frontend or backend.

Did you ever want a mock http server to test your front end or whatever? TestRoute will help you next time!

## How it works

You run the command:

```sh
testroute
```

You answer some questions:

```txt
> What is the path of your route? /api/user/4
> What HTTP method should listen to? GET
> What should be the response HTTP status? 200
> What should be the response? (Write a file path or leave empty for none) ./response.json
> There's any delay that you want on the route? (in seconds) 5
```

The server is up and ready to handle your requests!

```txt
Server is running on http://localhost:9999
Available routes:
  GET /api/users/4 - Status: 200
```

## Features

- Interactive-first, so you don't need to figure out arguments or flags right ahead
- Batteries-included: set the http method, status code, response body, delay and wildcard route (e.g. `/user/:id`)
- Highly configurable: you can create a set of mock routes one time and use it everywhere
- Intuitive and friendly with prompt autocompletion

## Usage

First, you need to install TestRoute:

```sh
cargo install testroute
```

Installing it via Cargo is the only way now, but the prebuilt binaries are coming soon.

### Getting started

TestRoute aims to make your life easier by being friendly and on point.

To start using it you just need to run the following command:

```sh
testroute
```

<div align="center">
<br/>

![TestRoute demonstration](.docs/demo-testroute.gif)

</div>

### HTTP file

You can define a set of mock routes in a file and TestRoute will serve it to you.

Create a `.http` file and copy the text below into it.

```txt
GET /foo 200
Delay=1

{
  "foo": true
}

POST /bar 400

{
  "bar": true,
  "message": "Back end with JS not allowed :p"
}

GET /api/users/:id 200

{
  "hello": "world!"
}
```

Run this command:

```sh
testroute --import path/to/routes.http
```

### Scripting

Want to create an automation with TestRoute? You can!

```sh
testroute -h
```

###### Flags

- `-p --path` to specify the route
- `-m --method` to set the http method
- `-s --status` to define the response status code
- `-r --response` to specify the response body
- `-d --delay` to simulate a latency (seconds)
- `-i --import` to import the configuration file
- `-h --help` to print the help menu
- `-V --version` to print the version

###### Examples

Setting all the flags:

```sh
testroute -p /api/users -m GET -s 200 -d 0 -r ""
# testroute --path /api/users --method GET --status 200 --delay 0 --response ""
```

Predefining route and method, finishing interactively:

```sh
testroute -p /api/users/:id -m GET

# GET http://localhost:9999/api/users/1234 should work
# and http://localhost:9999/api/users/whatever as well
```

Executing a set of routes in a file:

```sh
testroute --import mock-routes.http
```

## Contributing

Feel free to contribute, opening an issue to report a bug or suggesting a CLI change, an improvement or a new feature.

### How to contribute

1. Fork this repository
2. Clone your fork on your machine
3. Make your changes, commit and push them
4. Open a pull request (write a descriptive message about what you changed)
