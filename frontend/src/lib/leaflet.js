/** @import {LatLngTuple} from  "leaflet" */
import { coord_to_leaflet } from "./draw";

const MAP_PAD = 200;

export function map_bounds_to_leaflet(bounds) {
	// todo: scale MAP_PAD if number of cards is small
	return [
		coord_to_leaflet(bounds[0][0] - MAP_PAD, bounds[0][1] - MAP_PAD),
		coord_to_leaflet(bounds[1][0] + MAP_PAD, bounds[1][1] + MAP_PAD),
	];
}
/**
 * @param {number[][]} bounds
 * @returns {LatLngTuple}
 */
export function bounds_center(bounds) {
	let b = [
		coord_to_leaflet(bounds[0][0], bounds[0][1]),
		coord_to_leaflet(bounds[1][0], bounds[1][1]),
	];
	return [b[0][0] / 2 + b[1][0] / 2, b[0][1] / 2 + b[1][1] / 2];
}
