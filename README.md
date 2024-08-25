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
> What is the path of your route?: /api/user/4
> What HTTP method should listen to?: GET
> What should be the response HTTP status?: 200
> What should be the response? (Write a file path or leave empty for none): ./response.json
```

The server is up and ready to handle your requests!

```txt
> Please test your route at: http://localhost:9999/api/user/4
```

## Features

- Interactive-first, so you don't need to figure out arguments or flags right ahead
- Batteries-included: set the http method, status code, response body, delay and wildcard route (e.g. `/user/:id`)
- Highly configurable: you can create a set of mock routes one time and use it everywhere (not exactly how it works)
