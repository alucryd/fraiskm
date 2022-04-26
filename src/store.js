import { getMonth, getYear } from "date-fns";
import { range } from "lodash-es";
import { get, writable } from "svelte/store";

export const ready = writable(false);

export const user = writable(undefined);

export const drivers = writable([]);
export const vehicles = writable([]);
export const addresses = writable([]);
export const journeys = writable([]);
export const totals = writable([]);

export const getDriverById = (id) => get(drivers).find((driver) => driver.id == id);
export const getVehicleById = (id) => get(vehicles).find((vehicle) => vehicle.id == id);
export const getAddressById = (id) => get(addresses).find((address) => address.id == id);

export const years = range(2020, new Date().getFullYear() + 1);
export const months = [
  [1, "Janvier"],
  [2, "Février"],
  [3, "Mars"],
  [4, "Avril"],
  [5, "Mai"],
  [6, "Juin"],
  [7, "Juillet"],
  [8, "Août"],
  [9, "Septembre"],
  [10, "Octobre"],
  [11, "Novembre"],
  [12, "Décembre"],
];

const now = new Date();

export const currentYear = writable(getYear(now));
export const currentMonth = writable(getMonth(now) + 1);
export const currentDriverId = writable(undefined);

export const isDriverModalOpen = writable(false);
export const isVehicleModalOpen = writable(false);
export const isAddressModalOpen = writable(false);
export const isJourneyModalOpen = writable(false);
export const isJourneyDuplicateModalOpen = writable(false);
