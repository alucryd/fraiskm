import { GraphQLClient, gql } from "graphql-request";

import { user } from "./store.js";

const endpoint = "/graphql";
const graphQLClient = new GraphQLClient(endpoint);

export const passwordLowercaseRegex = /(?=.*[a-z])/;
export const passwordUppercaseRegex = /(?=.*[A-Z])/;
export const passwordNumberRegex = /(?=.*[0-9])/;
export const passwordSpecialCharacterRegex = /(?=.*[ !"#$%&'()*+,./:;<=>?@^_`{|}~\[\]\\-])/;
export const passwordLengthRegex = /(?=.{8,24})/;

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

  await graphQLClient.request(mutation);
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

export async function updateUsername(newUsername, password) {
  const mutation = gql`
    mutation UpdateUsername($newUsername: String!, $password: String!) {
      updateUsername(newUsername: $newUsername, password: $password)
    }
  `;

  const variables = {
    newUsername,
    password,
  };
  await graphQLClient.request(mutation, variables);
}

export async function updatePassword(password, newPassword) {
  const mutation = gql`
    mutation UpdatePassword($password: String!, $newPassword: String!) {
      updatePassword(password: $password, newPassword: $newPassword)
    }
  `;

  const variables = {
    password,
    newPassword,
  };
  await graphQLClient.request(mutation, variables);
}
