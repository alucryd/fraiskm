import { GraphQLClient } from "graphql-request";

const endpoint = "/graphql";
export const graphQLClient = new GraphQLClient(endpoint);
