import { GraphQLClient, gql } from "graphql-request";

import { user } from "./store.js";

const endpoint = "/graphql";
const graphQLClient = new GraphQLClient(endpoint);

export async function signin(username, password) {
  const mutation = gql`
    mutation Signin($username: String!, $password: String!) {
      signin(username: $username, password: $password) {
        username
      }
    }
  `;

  const variables = {
    username,
    password,
  };
  const data = await graphQLClient.request(mutation, variables);
  user.set(data.signin);
}

export async function signout() {
  const mutation = gql`
    mutation {
      signout
    }
  `;

  const data = await graphQLClient.request(mutation);
  user.set(null);
}

export async function signup(username, password) {
  const mutation = gql`
    mutation Signup($username: String!, $password: String!) {
      signup(username: $username, password: $password) {
        username
      }
    }
  `;

  const variables = {
    username,
    password,
  };
  const data = await graphQLClient.request(mutation, variables);
  user.set(data.signup);
}
