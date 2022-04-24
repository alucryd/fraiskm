import { gql } from "graphql-request";

import { graphQLClient } from "./graphql.js";
import { user } from "./store.js";

export const passwordLowercaseRegex = /(?=.*[a-z])/;
export const passwordUppercaseRegex = /(?=.*[A-Z])/;
export const passwordNumberRegex = /(?=.*[0-9])/;
export const passwordSpecialCharacterRegex = /(?=.*[ !"#$%&'()*+,./:;<=>?@^_`{|}~[\]\\-])/;
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

export async function createDriver(driver) {
  const mutation = gql`
    mutation CreateDriver(
      $name: String!
      $limitDistance: Boolean!
      $defaultVehicleId: ID
      $defaultFromId: ID
      $defaultToId: ID
    ) {
      createDriver(
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
}

export async function deleteDriver(driver) {
  const mutation = gql`
    mutation DeleteDriver($id: ID!) {
      deleteDriver(id: $id)
    }
  `;

  await graphQLClient.request(mutation, driver);
}

export async function createVehicle(vehicle) {
  const mutation = gql`
    mutation CreateVehicle($model: String!, $horsepower: Int!, $electric: Boolean!, $vehicleType: Int!) {
      createVehicle(model: $model, horsepower: $horsepower, electric: $electric, vehicleType: $vehicleType)
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

export async function createAddress(address) {
  const mutation = gql`
    mutation CreateAddress($title: String!, $label: String!, $addressType: Int!) {
      createAddress(title: $title, label: $label, addressType: $addressType)
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

export async function createDistance(distance) {
  const mutation = gql`
    mutation CreateDistance($fromId: ID!, $toId: ID!, $meters: Int!) {
      createDistance(fromId: $fromId, toId: $toId, meters: $meters)
    }
  `;

  await graphQLClient.request(mutation, distance);
}

export async function updateDistance(distance) {
  const mutation = gql`
    mutation UpdateDistance($fromId: ID!, $toId: ID!, $meters: Int!) {
      updateDistance(fromId: $fromId, toId: $toId, meters: $meters)
    }
  `;

  await graphQLClient.request(mutation, distance);
}

export async function createJourney(journey) {
  const mutation = gql`
    mutation CreateJourney(
      $driverId: ID!
      $vehicleId: ID!
      $fromId: ID!
      $toId: ID!
      $date: String!
      $meters: Int!
      $roundTrip: Boolean!
    ) {
      createJourney(
        driverId: $driverId
        vehicleId: $vehicleId
        fromId: $fromId
        toId: $toId
        date: $date
        meters: $meters
        roundTrip: $roundTrip
      )
    }
  `;

  await graphQLClient.request(mutation, journey);
}

export async function updateJourney(journey) {
  const mutation = gql`
    mutation UpdateJourney(
      $id: ID!
      $driverId: ID!
      $vehicleId: ID!
      $fromId: ID!
      $toId: ID!
      $date: String!
      $meters: Int!
      $roundTrip: Boolean!
    ) {
      updateJourney(
        id: $id
        driverId: $driverId
        vehicleId: $vehicleId
        fromId: $fromId
        toId: $toId
        date: $date
        meters: $meters
        roundTrip: $roundTrip
      )
    }
  `;

  await graphQLClient.request(mutation, journey);
}

export async function deleteJourney(journey) {
  const mutation = gql`
    mutation DeleteJourney($id: ID!) {
      deleteJourney(id: $id)
    }
  `;

  await graphQLClient.request(mutation, journey);
}
