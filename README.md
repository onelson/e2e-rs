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
> In *this iteration*, carried forward from the [master branch], the experiment
> was to replace the REST API with [GraphQL](http://graphql.org/) which naturally 
> meant a fair amount of refactor to support.

## Aims

Many of the original items for this section called out in earlier two iterations,
([wasm-bindgen branch] and [master branch]) were really specific to working in
terms of distinct endpoints and payload shapes.

GraphQL puts a slightly different spin on things so we're going to reframe the
aims this time around. Put more plainly, we aim to:

- conform both backend (rust) and frontend (typescript) to a common schema file.

## The Moving Parts

This version of the project involves far fewer individual sub-projects than any
others, in some small part due to how GraphQL controls what the exhaustive client
API looks like from the server-side, and Apollo's codegen tooling produces code
specific to usage. This means, the client code isn't something we can package up
for general use as we did in the previous iterations.

So, the sub-projects we have this time around are:

- e2e-server (rust): still using actix-web, but now also [juniper] and
  [juniper-from-schema] for the GraphQL support.
- web-frontend (js): as in the [master branch] this is a plain CRA project. This
  time, we add `apollo` et al for a client to talk to the backend.

Despite the lower package count, the overall implementation details are more
complex on the backend than before. Less so on the frontend, though there was
some configuration complexity there.

---

Before we move on to the nuts and bolts, here's a little overview of how graphql
operates.

The server exposes a single endpoint `/graphql` which all requests are POSTed to.
The handler for this endpoint accepts a `GraphQLRequest` (a json representation
for a GraphQL _operation_) and executes it in a GraphQL Engine built based on the
schema.

Instead of there being distinct endpoints for handling requests, GraphQL models
things in terms of "fields" which fall into one of two categories:

- Query (read-only fields)
- Mutation (write fields)

In the backend, these *fields* correspond to methods on structs (much like how
http handlers represent endpoints as functions).
This concept of fields continues down through to the objects included in the
responses to queries, which is to say every item that appears in the response is
comprised of a set of fields, each backed by a method call on a given struct.


On the frontend, there are a couple different ways to talk to the server.
You start with a client instance which can be used directly or via react hooks.

When properly configured, editors will be able to do introspection inside queries
(written as `gql` strings) to give typing feedback for fields according to the
schema.
Outside of the `gql` strings, typescript definitions are needed. These are
generated *based on a combination of the queries written for the project and the
schema*.


## Tools and Building

This section describes the various tools you'd need to be able to build and run
this project.

- https://rustup.rs/ (install this to get the latest stable toolchain; minimum version 1.39)
- https://nodejs.org/en/ (download, unpack, and add the bin dir to your PATH, I used 10.16.3 LTS)

Once you've got all this stuff installed, you should be able to run the following
in two different shells:

In one shell, from the `e2e-server` sub-directory, run

```shell script
$ env DATA_DIR=.. cargo run
```

and in another, from the `web-frontend` directory, run

```shell script
$ npm run start
```

## Developer Notes

This section outlines quirks and workarounds needed to put this together.

### `e2e-server`

The server uses [juniper] to build a GraphQL service this time. The main changes
required for this are found in the new `schema.rs` module.

The idea is we write up the full API and the types involved in a schema file
(using graphql schema language).
Once we have a schema, we load it with [juniper-from-schema] which produced
juniper calls to match.

The output produced by the macros in [juniper-from-schema] include trait
implementations for types which we must make present in the same module.
It also defines some traits which *we are responsible for implementing*.
It's important that these types are *all in the same module* since the macro
assumes this will be the case.

Most of the generated output is based on names being *aligned* between the
graphql schema and the rust types.

Consider the following schema definition:

```graphql
schema {
    query: Query
}

type Query {
    allMessages: [ChatLogEntry!]! @juniper(ownership: "owned")
}

```

The name `Query` is significant for the macro since it is used as a prefix in
the `QueryFields` trait, but also there are other traits which are
*automatically implemented for us* in the output so everything needs to align
*just right*.

```rust
pub struct Query;

impl QueryFields for Query {
    fn field_all_messages<'a>(
        &self,
        executor: &juniper::Executor<'a, Context>,
        _trail: &self::QueryTrail<'_, ChatLogEntry, juniper_from_schema::Walked>,
    ) -> juniper::FieldResult<Vec<ChatLogEntry>> {
        Ok(executor.context().chat_storage.all_messages())
    }
}
```

Each field method (I *think* these are referred to as **field resolvers**) is
given access to a `Context` type plays a role similar to `Data` in actix-web.
This is the way we can prepare and expose shared data with these field resolvers.

For example, we could provide a getter for a database connection.
In this toy application, we use `Context` to get access to our in-process chat
storage, and name generator.

### `web-frontend`

As in the [master branch], we're using a vanilla CRA setup. We do have some new
dependencies to talk about, however.

Here's the breakdown of what's been added:

#### New Dependencies

- `apollo`: this is for the apollo cli tool we'll use for generating typescript
  based on a graphql schema.
- `apollo-boost`: this is the apollo client we'll use to talk to the backend, as
  well as stuff like the `gql` "tag" which we use to write graphql queries.
- `@apollo/react-hooks`: this gives us a provider component for using an apollo
  client, and hooks to go along with it.

#### New npm tasks

- `gql:clean`: destroys and recreates the directory where our generated files
  will end up.
- `gql:prep-schema`: this one is odd, and someday may not be needed but it makes
  a copy of the schema in the server dir, stripping out some stuff that isn't
  legal in the process. This **cleaned version of the schema** is what all the
  frontend stuff will look at.
- `gql:apollo`: runs the typescript codegen based on the **cleaned schema**.
- `codegen`: this one runs through all the above tasks in series, and is added as
  a first step for `build`, `test`, and `run`.

#### New Configuration

We already have our webpack proxy configured to map `/api` to our backend, but
in addition to this, we need to add some config for apollo.

- `.graphqlconfig`: this is mainly to configure editors that want to probe the
  schema or run queries. This enables query introspection and type hints in
  intellij, for example.
- `apollo.config.js`: this provides defaults to the apollo cli. Importantly it
  specifies the schema file to look at - the **cleaned** schema in the generated
  output.

The apollo config lists the generated **cleaned** schema file in the exclude
patterns, which seems odd but I think it's needed because the schema exists
beneath `src/` and is automatically re-read after the initial schema load (as
per the `localSchemaFile` entry). Without this workaround, you'll see errors
during codegen talking about types already being registered.

> The whole copying/modifying/cleaning of the schema file is currently needed
> because of the way `juniper-from-schema` uses sort of "fake" schema directives
> to influence the type signatures in the traits it generates.
> This is a workaround, and may not always be needed.
>
> For details, see: <https://github.com/davidpdrsn/juniper-from-schema/issues/103>

#### Implementation Notes

So far, we've mainly talked about tooling and build topics for the frontend.
This section talks about the way all this stuff comes together in the
application code itself.

Largely I followed Apollo's [Getting Started][apollo-getting-started] guide, so
please refer to it for info on the client, provider and hooks.

There were some quirks to draw attention to with the typescript setup, so that's
where we'll focus for now. I got a lot of clues from
<https://www.leighhalliday.com/generating-types-apollo> but some of the info
didn't seem to work well perhaps because of CRA's constraints or simply due to
the passage of time.

There are a set of conventions and caveats with this setup, so I'll try and
call them out.

First off, since the typescript we generate is based on queries, this adds a
special requirement to how we write them. Namely, *operations* (which is an
*optional label* you can add to the outermost level of the query) is
**no longer optional** for us.

For example, consider the `READ_MESSAGES` query, used in `ChatList.tsx`

```typescript jsx
import { gql } from "apollo-boost";
import { ReadMessages } from "../_gql/generated/ReadMessages";

const READ_MESSAGES = gql`
  query ReadMessages {
    allMessages {
      msg {
        author
        text
      }
      timestamp
    }
  }
`;

// ...

const { loading, error, data } = useQuery<ReadMessages>(READ_MESSAGES);
```

So, the type we import from `../_gql/generated/ReadMessages` is named
specifically for the name of the operation specified in the query as
`query ReadMessages {`.

The generated type represents the ultra-specific payload shape requested in the
query, and so as you write new queries, or modify existing ones you'll have to
re-run `npm run codegen` to update the types to match.

In addition to this, each operation name *must be unique*. This is why I sort of
fell into defining all the queries as constants in a single module.

Looking at the generated typescript brings us to another caveat:

```typescript
export interface ReadMessages_allMessages_msg {
  __typename: "Message";
  author: string;
  text: string;
}

export interface ReadMessages_allMessages {
  __typename: "ChatLogEntry";
  msg: ReadMessages_allMessages_msg;
  timestamp: E2eDateTimeUtc;
}

export interface ReadMessages {
  allMessages: ReadMessages_allMessages[];
}
```

The schema defines a custom scalar type for `DateTimeUtc` which is what
`juniper` uses as a part of its `chrono` support, but none of the graphql tech
is able to know what the *actual type* is for it. We need to provide a type
definition for that symbol in order for our app to compile.

Since we know json encoded dates are `string`, we add the following to
`src/_gql/customScalars.d.ts`:

```typescript
/**
 * An ISO Date time.
 */
type E2eDateTimeUtc = string;
```

There's a block comment in this file mentioning that there
**should not be any imports or exports** in this file. This is important to make
the symbol we defined appear "in scope" without importing it, as we need it to
be in the generated typescript code.
*There'd be no easy way to inject an import into those generated modules.*

Typescript files without imports/exports in them are treated as "scripts" rather
than "modules" by the compiler, which apparently means the symbols are global.

The `E2e` prefix on that symbol was put there by the apollo codegen tool based
on flags passed to it. The reason we specify a project prefix to use for these
custom scalars during codegen is as a safety precaution (since the symbols will
be global, we don't want to stumble into collisions).

So, when enabling more of juniper's custom scalars, you need to provide a
mapping for it in that special "global types" file.

I think the only other oddity during walking through all this was in the
codegen npm task.

Here's the set of flags we supply (remembering some defaults are supplied by
the `apollo.config.js`):

```shell script
$ apollo codegen:generate \
  --target typescript \
  --tsFileExtension=d.ts \
  --passthroughCustomScalars \
  --customScalarsPrefix=E2e \
  --outputFlat src/_gql/generated
```

I noticed that something with tslint (rules applied by CRA) broke unless I set
the `tsFileExtension` to `d.ts` instead of the regular `.ts`.
The complaint talked about how the TS usage was not allowed with
`isolatedModules=true` and to set it to `false`, which I did in `tsconfig.json`.

Unfortunately this setting appears to be ignored by some of the tech running as
a part of `react-scripts` which means the app can compile, but `npm run start`
will show a compile error at runtime (after the app has already started??)

I stumbled into changing the extension as a workaround, which works while
`isolatedModules` is still `true`. I'm not sure why this works or why it matters.

In summary:

- Every operation *must have a name*.
- Every operation name *must be unique*.
- You must run `npm run codegen` after changing the schema or your queries.
- Custom scalars in the schema must be defined in `src/_gql/customScalars.d.ts`.


[wasm-bindgen branch]: https://github.com/onelson/e2e-rs/tree/wasm-bindgen
[master branch]: https://github.com/onelson/e2e-rs/tree/master
[graphql branch]: https://github.com/onelson/e2e-rs/tree/graphql
[grpc branch]: https://github.com/onelson/e2e-rs/tree/grpc
[juniper]: https://crates.io/crates/juniper
[juniper-from-schema]: https://crates.io/crates/juniper-from-schema
[apollo-get-started]: https://www.apollographql.com/docs/react/get-started/

