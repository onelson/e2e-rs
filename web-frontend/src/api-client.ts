import ApolloClient from "apollo-boost";
import { gql } from "apollo-boost";

export function getClient(): ApolloClient<any> {
  return new ApolloClient({
    uri: "/api/graphql"
  });
}

export const CREATE_MESSAGE = gql`
  mutation CreateMessage($msg: NewMessage!) {
    createMessage(message: $msg)
  }
`;

export const READ_MESSAGES = gql`
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

export const GENERATE_USERNAME = gql`
  mutation GenerateUsername {
    username: getUsername
  }
`;
