module.exports = {
  client: {
    excludes: ["node_modules/*", "src/_gql/generated/*"],
    tagName: "gql",
    service: {
      name: "MessagesAPI",
      localSchemaFile: "../schema.graphql"
    }
  }
};
