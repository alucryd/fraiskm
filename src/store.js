import { writable } from "svelte/store";

export const user = writable(undefined);

export const addresses = writable([]);
export const vehicles = writable([]);
export const drivers = writable([]);

export const isAddressModalOpen = writable(false);
export const isVehicleModalOpen = writable(false);
export const isDriverModalOpen = writable(false);
