import { GraphQLClient, gql } from "graphql-request";

import { addresses, drivers, user, vehicles } from "./store.js";

const endpoint = "/graphql";
const graphQLClient = new GraphQLClient(endpoint);

export async function me() {
  const query = gql`
    {
      me {
        username
      }
    }
  `;

  const data = await graphQLClient.request(query);
  user.set(data.me);
}

export async function getAddresses() {
  const query = gql`
    {
      addresses {
        id
        title
        label
        addressType
      }
    }
  `;

  const data = await graphQLClient.request(query);
  addresses.set(data.addresses);
}

export async function getVehicles() {
  const query = gql`
    {
      vehicles {
        id
        model
        horsepower
        electric
        vehicleType
      }
    }
  `;

  const data = await graphQLClient.request(query);
  vehicles.set(data.vehicles);
}

export async function getDrivers() {
  const query = gql`
    {
      drivers {
        id
        name
        limitDistance
        defaultVehicleId,
        defaultFromId,
        defaultToId,
      }
    }
  `;

  const data = await graphQLClient.request(query);
  drivers.set(data.drivers);
}
