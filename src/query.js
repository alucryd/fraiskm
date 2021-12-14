import { GraphQLClient, gql } from "graphql-request";

import { addresses, drivers, user, vehicleTypes, vehicles } from "./store.js";

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

  try {
    const data = await graphQLClient.request(query);
    user.set(data.me);
  } catch (error) {
    console.log(error);
  }
}

export async function getAddresses() {
  const query = gql`
    {
      addresses {
        id
        title
        label
      }
    }
  `;

  const data = await graphQLClient.request(query);
  addresses.set(data.addresses);
}

export async function getDrivers() {
  const query = gql`
    {
      drivers {
        id
        name
        limitDistance
      }
    }
  `;

  const data = await graphQLClient.request(query);
  drivers.set(data.drivers);
}

export async function getVehicles() {
  const query = gql`
    {
      vehicles {
        id
        model
        horsepower
        electric
        vehicleTypeId
      }
    }
  `;

  const data = await graphQLClient.request(query);
  vehicles.set(data.vehicles);
}

export async function getVehicleTypes() {
  const query = gql`
    {
      vehicleTypes {
        id
        label
      }
    }
  `;

  const data = await graphQLClient.request(query);
  vehicleTypes.set(data.vehicleTypes);
}
