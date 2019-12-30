module.exports = {
  client: {
    excludes: ["node_modules/*"],
    tagName: "gql",
    service: {
      name: "MessagesAPI",
      localSchemaFile: "../messages.graphql"
    }
  }
};
