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
  user.set(undefined);
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

export async function normalizeAddress(label) {
  const mutation = gql`
    mutation NormalizeAddress($label: String!) {
      normalizeAddress(label: $label)
    }
  `;

  const variables = {
    label,
  };
  const data = await graphQLClient.request(mutation, variables);
  return data.normalizeAddress;
}

export async function addAddress(address) {
  const mutation = gql`
    mutation AddAddress($title: String!, $label: String!, $addressType: Int!) {
      addAddress(title: $title, label: $label, addressType: $addressType)
    }
  `;

  await graphQLClient.request(mutation, address);
}

export async function updateAddress(address) {
  const mutation = gql`
    mutation UpdateAddress($id: ID!, $title: String!, $label: String!, $addressType: Int!) {
      updateAddress(id: $id, title: $title, label: $label, addressType: $addressType)
    }
  `;

  await graphQLClient.request(mutation, address);
}

export async function deleteAddress(address) {
  const mutation = gql`
    mutation DeleteAddress($id: ID!) {
      deleteAddress(id: $id)
    }
  `;

  await graphQLClient.request(mutation, address);
}

export async function addVehicle(vehicle) {
  const mutation = gql`
    mutation AddVehicle($model: String!, $horsepower: Int!, $electric: Boolean!, $vehicleType: Int!) {
      addVehicle(model: $model, horsepower: $horsepower, electric: $electric, vehicleType: $vehicleType)
    }
  `;

  await graphQLClient.request(mutation, vehicle);
}

export async function updateVehicle(vehicle) {
  const mutation = gql`
    mutation UpdateVehicle($id: ID!, $model: String!, $horsepower: Int!, $electric: Boolean!, $vehicleType: Int!) {
      updateVehicle(id: $id, model: $model, horsepower: $horsepower, electric: $electric, vehicleType: $vehicleType)
    }
  `;

  await graphQLClient.request(mutation, vehicle);
}

export async function deleteVehicle(vehicle) {
  const mutation = gql`
    mutation DeleteVehicle($id: ID!) {
      deleteVehicle(id: $id)
    }
  `;

  await graphQLClient.request(mutation, vehicle);
}

export async function addDriver(driver) {
  const mutation = gql`
    mutation AddDriver(
      $name: String!
      $limitDistance: Boolean!
      $defaultVehicleId: ID
      $defaultFromId: ID
      $defaultToId: ID
    ) {
      addDriver(
        name: $name
        limitDistance: $limitDistance
        defaultVehicleId: $defaultVehicleId
        defaultFromId: $defaultFromId
        defaultToId: $defaultToId
      )
    }
  `;

  await graphQLClient.request(mutation, driver);
}

export async function updateDriver(driver) {
  const mutation = gql`
    mutation UpdateDriver(
      $id: ID!
      $name: String!
      $limitDistance: Boolean!
      $defaultVehicleId: ID
      $defaultFromId: ID
      $defaultToId: ID
    ) {
      updateDriver(
        id: $id
        name: $name
        limitDistance: $limitDistance
        defaultVehicleId: $defaultVehicleId
        defaultFromId: $defaultFromId
        defaultToId: $defaultToId
      )
    }
  `;

  await graphQLClient.request(mutation, driver);
  // graphQLClient
  //   .request(mutation, driver)
  //   .then((data) => console.log(data))
  //   .then((error) => console.log(error));
}

export async function deleteDriver(driver) {
  const mutation = gql`
    mutation DeleteDriver($id: ID!) {
      deleteDriver(id: $id)
    }
  `;

  await graphQLClient.request(mutation, driver);
}
