## Aims 

When building http server and client apis it can sometimes be a challenge to 
keep request and response payload shapes aligned.

This project aims to explore options to build both client and server with a
shared collection of types. It is my hope that we'll be able to publish client
libraries to match servers in lock-step by having a mutual dependency on a
rust crate which contains all the "shapes" that can be seen from each side.

The high-level goal here is to:

- build a json-based web api server
- build a client to match
- import and use the client from a react app
- if possible, export Typescript info with the client
- static analysis (the rust type system) to enforce all projects are aligned at any given time

## The Moving Parts

There's some complexity in how this all fits together so here's the gist.

In the top level of the repo, there are 4 distinct sections:

- `e2e-core` (rust): common types shared between client and server
- `e2e-server` (rust): an `actix-web` api server
- `e2e-client` (rust, js): this rust/js hybrid project uses `wasm-bindgen`/
  `webpack` to produce a js package containing the http client code.
- `web-frontend` (js): this is a `create-react-app` project with some tweaks to
  add wasm support. This project depends on the `e2e-client` package.

## Tools and Building

This section describes the various tools you'd need to be able to build and run
this project.

- https://rustup.rs/ (install this to get the latest stable toolchain, I used 1.38)
- https://nodejs.org/en/ (download, unpack, and add the bin dir to your PATH, I used 10.16.3 LTS)
- https://rustwasm.github.io/wasm-pack/installer/ (just be sure to install this _after_ installing rust)

Once you've got all this stuff installed, you should be able to run the following:

```shell script
$ cargo build -p e2e-server
$ cd e2e-client
$ npm i
$ npm run build
$ cd ../web-frontend
$ npm i
```

Ordering is important here in so much as `web-frontend` won't be able to install
`e2e-client` unless `e2e-client` has already been built.

At this point you should be able to run the project.

In one shell, from the project root directory, run 

```shell script
$ cargo run -p e2e-server
```

and in another, from the `web-frontend` directory, run

```shell script
$ npm run start
```
 
## Developer Notes

This section describes what was done to put all the pieces together, more or less
discussing _just the happy path_ used to bootstrap the project from the ground up.

The `e2e-core` and `e2e-server` crates are not really that notable. They are
written plainly as any other conventional rust crates are, and so we'll move 
directly on to the other sections of the project.

### e2e-client

The HTTP client follows the [wasm-bindgen Hello World] guide closely, mainly
adjusting things like references the name of the crate.

The _Hello World_ setup encourages the use of `webpack` for the project build,
which initially I thought was not appropriate for the ultimate goal of importing
from yet another webpack-based project (the react app), but it turns out this is
actually fine, good, and correct to do.

The guide steps you through the creation of a rust crate, the addition of some
attributes to help annotate things which will be exported to the JS API, and
finally the addition of a small number of npm/js related project files to help
with the build. 

My early attempts to follow this guide failed when importing the client in the
react app, but it seems all the issues were actually on the app project
configuration, not here in the `e2e-client` crate.

One _gotcha_ to note is that when building the client with `npm run build`, the
npm packaging scripts can succeed even if the `cargo build` they trigger fails.

Watch for build failures when running the npm build, or better yet run 
`cargo check` or `cargo build` ahead of time. It may be good to add these as a
pre-flight step for the `build` script entry in the client's `package.json`.


### web-frontend

The react app was bootstrapped with `create-react-app` (CRA) with the
`--typescript` flag enabled. As such, files in the project that use js/jsx
extension are treated as plain javascript and those with ts/tsx are additionally
checked with the Typescript compiler.

In order to incorporate the `e2e-client` package as a dependency, I had to make
some tweaks to how the project is built.

Making customizations to CRA apps can be tricky since a large part of the goal
of CRA is to hide configuration details. Internet research led me to
Preston Richey's [Up and Running with React + Rust + Wasm]
which describes much of the missing pieces I needed to get things working.

The main takeaways I took where:

- using [react-app-rewired] to modify the webpack config supplied by CRA (to add
  a wasm webpack loader).
- using `await import("e2e-client")` from inside an async function instead of a
  regular import.

The main webpack customizations I made for the project can be seen in 
[this diff][modifications].
  
The _async import_ was something I missed early on in my testing. I couldn't
quite reconcile the examples shown in the `wasm-bindgen` docs with how I should
be importing my client library from _yet another_ webpack project, but the blog
post by Preston Richey gave sufficient clues.

This is early still, but I'm likely going to try and provide
[helpers][async import helper] which will help to encourage the correct usage,
requiring developers to get access to the package via async functions.

[wasm-bindgen Hello World]: https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html
[modifications]: https://github.com/onelson/e2e-rs/commit/d51acb6a4460c81efba84dbc4f1f980704c76f89
[async import helper]: https://github.com/onelson/e2e-rs/blob/6265356f1ced493c5d4fd45f037c3ea1231114ca/web-frontend/src/api-client.js
[Up and Running with React + Rust + Wasm]: https://prestonrichey.com/blog/react-rust-wasm
[react-app-rewired]: https://github.com/timarney/react-app-rewired