import { writable } from "svelte/store";

export const user = writable(null);

export const vehicleTypes = writable([]);
export const drivers = writable([]);
export const vehicles = writable([]);
export const addresses = writable([]);
