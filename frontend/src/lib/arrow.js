/** @import {LatLngTuple, LatLngBoundsExpression} from  "leaflet" */

import { SVG_NS } from "@/lib/card";

const STROKE = 8;

// arrowhead with width = 30 and height = 45
const ARROW = {
	path: "M 0,0 L 0,30 L 15,45 L 30,30 L 30,0 L 15,15 Z",
	cx: 15,
	cy: 22.5,
};

/**
 * Make svg arrow from center1 to center2.
 *
 * @param {string}      id
 * @param {LatLngTuple} center1
 * @param {LatLngTuple} center2
 * @param {string}      category_class
 * @returns {SVGElement}
 */
export function make_rumor_arrow(id, center1, center2, category_class) {
	let [x1, y1] = center1;
	let [x2, y2] = center2;

	let rect_width = Math.abs(y2 - y1);
	let rect_height = Math.abs(x2 - x1);

	let dx = x2 - x1;
	let dy = y2 - y1;

	// todo: calculate not just middle between centers, but center between
	// intersections with cards edges
	let [cx, cy] = [rect_width / 2, rect_height / 2];
	let rad = Math.atan2(y1 - y2, x1 - x2);
	let deg = (rad * 180) / Math.PI;

	if (dx * dy < 0) {
		// when `dx * dy < 0` one coordinate increases, other decreases, so `dx` and `dy` has different signs
		// top left -> bottom right
		x1 = 0;
		y1 = 0;
		x2 = rect_width;
		y2 = rect_height;
	} else {
		// bottom left -> top right
		x1 = 0;
		y1 = rect_height;
		x2 = rect_width;
		y2 = 0;
	}

	let e = document.createElementNS(SVG_NS, "svg");
	e.setAttribute("xmlns", SVG_NS);
	e.setAttribute("viewBox", `0 0 ${rect_width} ${rect_height}`);
	e.setAttribute("class", category_class);

	e.innerHTML = `<line id="${id}" x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" class="arrow" stroke-width="${STROKE}" />
		<path d="${ARROW.path}" transform="translate(${cx - ARROW.cx}, ${cy - ARROW.cy}) rotate(${deg}, ${ARROW.cx}, ${ARROW.cy})" class="arrow" />`;
	return e;
}

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
