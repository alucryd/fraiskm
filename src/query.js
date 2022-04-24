import { gql } from "graphql-request";

import { graphQLClient } from "./graphql.js";
import { addresses, drivers, journeys, totals, user, vehicles } from "./store.js";

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

export async function getDrivers() {
  const query = gql`
    {
      drivers {
        id
        name
        limitDistance
        defaultVehicleId
        defaultFromId
        defaultToId
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
        vehicleType
      }
    }
  `;

  const data = await graphQLClient.request(query);
  vehicles.set(data.vehicles);
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

export async function getDistance(id0, id1) {
  const query = gql`
    query Distance($id0: ID!, $id1: ID!) {
      distance(id0: $id0, id1: $id1) {
        meters
      }
    }
  `;

  const variables = {
    id0,
    id1,
  };
  const data = await graphQLClient.request(query, variables);
  return data.distance.meters ? data.distance.meters : 0;
}

export async function getJourneys(driverId, year, month) {
  const query = gql`
    query Journeys($driverId: ID!, $year: Int!, $month: Int!) {
      journeys(driverId: $driverId, year: $year, month: $month) {
        id
        fromId
        toId
        driverId
        vehicleId
        date
        meters
        roundTrip
      }
    }
  `;

  const variables = {
    driverId,
    year,
    month,
  };
  const data = await graphQLClient.request(query, variables);
  journeys.set(data.journeys);
}

export async function getTotals(driverId, year) {
  const query = gql`
    query Total($driverId: ID!, $year: Int!) {
      totals(driverId: $driverId, year: $year) {
        vehicleId
        formula
        total
      }
    }
  `;

  const variables = {
    driverId,
    year,
  };
  const data = await graphQLClient.request(query, variables);
  totals.set(data.totals);
}
