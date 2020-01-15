> This project is not arranged in the usual fashion you might manage a software
> project.
> There are several branches, some built on top of others, but they are not meant
> to be viewed in terms of their history per se. It may be interesting to look at
> them as total diffs via a "compare" view, but the lineage of each branch isn't
> that important.
>
> Each of the branches named here represents a different approach, tech-wise, to
> solve the problem keeping server and client applications aligned via
> type-safety enforced by rust and typescript. 
> 
> In order of implementation, we've got:
>
> 1. [wasm-bindgen branch] - uses wasm-bindgen to export an HTTP client package.
>    Alignment is maintained by using rust itself to write the HTTP client,
>    mostly.
> 1. [master branch] - aka "wasm removal" dumps wasm-bindgen, but uses the 
>    `typescript-definitions` crate to export types used in a hand-written
>    Typescript HTTP client package. This is the version with the most manual
>    upkeep required.
> 1. [graphql branch] - uses `juniper` for the server, and `apollo` on the
>    client. Both server and client use codegen based on a common schema to
>    enforce alignment.
> 1. [grpc branch] - a similar approach to the graphql branch. Codegen is used
>    on a common `.proto` file to produce code for a `tonic` server and
>    `grpc-web` client.
> 

## Aims

The aim of each of these implementations is to drive conformance and
alignment using a core set of types, hopefully controlled from a single place in
the codebase.

The first two versions relied on rust structs to generate matching typescript
definitions. GraphQL used a schema file to generate traits on the server side,
and typescript on the client side.

This gRPC-based implementation, like GraphQL, uses a schema in the form of a
`.proto` file which is used to generate types for both client and server.

## The Moving Parts

The sub-projects we have this time around are:

- e2e-server (rust): no longer using actix-web like the other versions of this
  app. This time we're using [tonic], which is a hyper-based gRPC server framework.
- e2e-client (js): this npm project includes scripts for doing codegen via
  `protoc`, bundling the results and exporting them carefully with typescript
  definitions.
- web-frontend (js): as in the [master branch] this is a plain CRA project. No
  particularly exotic deps added here this time. The only "special" addition is
  `e2e-client`.
- envoy (docker): in the other versions, we could rely on webpack's proxy
  to connect the client and server, but this time we need a more specialized
  setup.


---

Before we move on to the nuts and bolts, here's a little overview of how gRPC
operates.

The server exposes endpoints based on the hierarchy: _package_, _service_ and
finally the _rpc_ methods it exposes.

For example, in this app we have a `Chatroom` service in a `chatroom` package,
and it exposes a `CreateMessage` rpc call.
All this is represented by the URI: `/chatroom.Chatroom/CreateMessage`.

Clients can POST "messages" to these rpc endpoints. The request bodies are often
blobs of binary data represented as [protobuf], which the service and client both
know how to encode and decode into structs or classes on either end.

The fact that the payloads are binary means they can be compressed aggressively, 
which can lower overall latency of our applications. It also means we can model
our data using slightly more interesting types than the standard json scalars. 

## Tools and Building


### Installation

This section describes the various tools you'd need to be able to build and run
this project.

- https://rustup.rs/ (install this to get the latest stable toolchain; minimum version 1.39)
- https://nodejs.org/en/ (download, unpack, and add the bin dir to your PATH, I used 10.16.3 LTS)

The first two items are the same as all the other versions of this app.
This time however, we've got a couple new items to install.
 
- `docker` and `docker-compose` (left as a task for the reader)
- https://github.com/protocolbuffers/protobuf/releases (this is the protobuf compiler)
  - download `protoc-3.11.2-<platform>.zip`... 
  - unpack it and make sure `bin/protoc` is added to your `PATH`
- https://github.com/grpc/grpc-web/releases (this is the protoc plugin for exporting a grpc client for use in a browser)
  - download `protoc-gen-grpc-web-1.0.7-<platform>`
  - rename it to `protoc-gen-grpc-web`
  - `chmod +x` the file
  - make sure it is added to your `PATH`

You will likely not need to run protoc directly, but it will be invoked during
the building of the `e2e-client` npm package. If your package fails to build
because of protoc-related issues, refer back to the docs for each of those
downloads. 

Also of interest is [BloomRPC] which is a gRPC GUI client.
While not required for running the project, it could certainly be useful for
hacking on it. 

### Running

Once you've got all this stuff installed, you should be able to get things up
and running.

In one shell, fire up `envoy` (our proxy). This allows the client to talk to the
server using http/1 like the browser often wants to.

To do this, from the project root, run

```shell script
$ docker-compose up
```


In another shell, from the `e2e-server` sub-directory, run

```shell script
$ env DATA_DIR=.. cargo run
```

In yet another shell, build the `e2e-client` package by switching to the
`e2e-client` directory and running:

```shell script
$ npm install
$ npm run build
```

then finally, from the `web-frontend` directory, run

```shell script
$ npm install
$ npm run start
```

At this point everything should be up and running and you should be able to mess
around with the app in your browser.

## Developer Notes

This section outlines quirks and workarounds needed to put this together.

### `e2e-client`

The `package.json` has script entry points listed which run `protoc` to generate
protobuf-based type definitions, and a client class. The `build` script runs the
codegen, and copies all the needed files into the `dist` directory, which is all
compiled down to es5 with `.d.ts` files to go along with.

This should ensure the client can be happily used from older projects that
aren't using Typescript yet.

This package explicitly selects some stuff to re-export from the generated
sources, but we could likely be less selective and just publish the whole thing.

### `e2e-server`

The server was built mostly by following the advice in
https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md

~~A few tweaks had to be made version-wise in the dependencies, but these tweaks~~
~~will likely be redundant in the next week or so after tonic 0.1.0 is officially~~
~~released.~~ 
~~The work in this repo currently targets the beta release.~~

The main deviations I had to make were to:

- ~~add a dependency on `bytes` at `0.4` rather than `0.5` as wanted by the latest `prost`.~~
- ~~hold back the version of `prost` to `0.5` instead of using `master` from git.~~
- added a dependency on `prost-types` which is needed when you use "extra" rich protobuf types (we're using `Timestamp`).

Other than ~~these tweaks~~ that extra dep, the tutorial proved accurate.
We use a macro in our `schema` module which includes code generated by the build
script, based on the `.proto` file.

Once the server code has been generated, we had to implement some traits on some
concrete types of our making, yadda yadda. This is all very similar to how things
worked in the [graphql branch] except that the code to implement was more just
for RPC calls, rather than individual fields all over the place (making the task
much more straightforward). Each RPC call is represented by a type for the
incoming message, the response message, and these are used in the signature for
a method on a trait representing the "service".

### `web-frontend`

As in the [master branch], we're using a vanilla CRA setup.

There's not a whole lot to talk about here.
We added a dependency on `e2e-client` and use it. Nothing fancy or special required.


### `envoy`

Something I was slow to realize was how `tonic`, the server, speaks **http/2**
only and grpc-web, because of browser limitations will often, but not always, send
requests using **http/1**. For this reason, the general rule is when trying to do
grpc in the browser you need a proxy or "bridge" to allow both protocols to be used
freely based on the needs of the individual client.

The recommended proxy (by google) is [envoy], but recent versions of nginx can
also be used.

See [State of gRPC in the browser] for background.

To start, I opted to go with the _more recommended_ envoy, and to help make
local dev more smooth, I added a `docker-compose.yaml` to help run it.

Quirk: since our rust server is not running in docker, I set up docker-compose
to use `"host"` as the network mode. This means the ports the docker container
bind to are not mapped as per usual. Ports are bound directly to the host
machine, ie "localhost".

Envoy is configured to take traffic at `localhost:8080` and forward it to 
`[::1]:50051` which is the ipv6 address the tonic service binds to.

The web frontend therefore uses `http://localhost:8080` to connect to the backend.

It looks as though host mode networking might not be needed with some more
cleverness, but it seems to work for me. I did note, however the guide from the 
grpc-web docs mentioned some issues with the networking on mac.

- https://github.com/grpc/grpc-web/issues/436

Still, the mac issue may not be a problem when running in host networking mode 
(which was not a part of the guide). I guess we'll see.

> Something to note about the proxy is if you are using [BloomRPC], you should
> not point it at the proxy. Instead you point it directly at the tonic server.
> Bloom will use a nodejs client (I assume), not grpc-web from the browser so the
> proxy is not needed.

#### Implementation Notes

I guess one thing to mention is how the protobuf "message" classes on the
frontend are a little clunky to work with. Fields are represented by getters and
setters. 

They all have a `toObject` method which returns a plain object version of the
class, as well as some helpers for comparing messages/fields to each other, or
converting to other types.

Another clunky aspect, though I get the motivation, is how even RPC methods that
don't take arguments per se expect a message as a parameter.
The rationale is for forwards/backwards compatibility where if all callers know
to pass a message to an RPC call, they can opt-in to setting certain fields as
those fields are introduced. _Okay, I guess._

All in all, the frontend looks very close to the [master branch] in terms of
client usage. The contracts are more or less the same, but this time we didn't
have to write the client code at all.

[wasm-bindgen branch]: https://github.com/onelson/e2e-rs/tree/wasm-bindgen
[master branch]: https://github.com/onelson/e2e-rs/tree/master
[graphql branch]: https://github.com/onelson/e2e-rs/tree/graphql
[grpc branch]: https://github.com/onelson/e2e-rs/tree/grpc
[tonic]: https://github.com/hyperium/tonic
[protobuf]: https://github.com/protocolbuffers/protobuf
[State of gRPC in the browser]: https://grpc.io/blog/state-of-grpc-web/
[Envoy]: https://www.envoyproxy.io/
[BloomRPC]: https://github.com/uw-labs/bloomrpc