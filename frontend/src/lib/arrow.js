/** @import {LatLngBoundsExpression} from  "leaflet" */

export const STROKE = 8;

/**
 * Add padding when bounds forms too thin rectangle.
 *
 * @param {number[][]} bounds
 * @returns {LatLngBoundsExpression}
 */
export function expand_thin_bounds(bounds) {
	const PAD = STROKE * 3;

	let [[x1, y1], [x2, y2]] = bounds;
	if (Math.abs(x2 - x1) < PAD) {
		if (x1 < x2) {
			x1 -= PAD;
			x2 += PAD;
		} else {
			x1 += PAD;
			x2 -= PAD;
		}
	} else if (Math.abs(y2 - y1) < PAD) {
		if (y1 < y2) {
			y1 -= PAD;
			y2 += PAD;
		} else {
			y1 += PAD;
			y2 -= PAD;
		}
	}

	return [
		[x1, y1],
		[x2, y2],
	];
}
