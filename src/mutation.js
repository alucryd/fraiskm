import { GraphQLClient, gql } from "graphql-request";

import { user } from "./store.js";

const endpoint = "/graphql";
const graphQLClient = new GraphQLClient(endpoint);

export async function login(username, password) {
  const mutation = gql`
    mutation Login($username: String!, $password: String!) {
      login(username: $username, password: $password) {
        username
      }
    }
  `;

  const variables = {
    username,
    password,
  };
  const data = await graphQLClient.request(mutation, variables);
  user.set(data.login);
}

export async function logout() {
  const mutation = gql`
    mutation {
      logout
    }
  `;

  const data = await graphQLClient.request(mutation);
  user.set(null);
}
