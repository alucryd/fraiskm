import { getMonth, getYear } from "date-fns";
import { get, writable } from "svelte/store";

export const ready = writable(false);

export const user = writable(undefined);

export const drivers = writable([]);
export const vehicles = writable([]);
export const addresses = writable([]);
export const journeys = writable([]);

export const getDriverById = (id) => get(drivers).find((driver) => driver.id == id);
export const getVehicleById = (id) => get(vehicles).find((vehicle) => vehicle.id == id);
export const getAddressById = (id) => get(addresses).find((address) => address.id == id);

const now = new Date();

export const journeyDriverId = writable(undefined);
export const journeyYear = writable(getYear(now));
export const journeyMonth = writable(getMonth(now) + 1);

export const isDriverModalOpen = writable(false);
export const isVehicleModalOpen = writable(false);
export const isAddressModalOpen = writable(false);
export const isJourneyModalOpen = writable(false);
export const isJourneyDuplicateModalOpen = writable(false);
