import { SVG_NS } from './card'

const STROKE = 8

/**
 * Make svg arrow from center1 to center2
 * @param  {string} id
 * @param  {import('leaflet').LatLngTuple} center1
 * @param  {import('leaflet').LatLngTuple} center2
 * @return {SVGElement}
 */
export function make_rumor_arrow(id, center1, center2) {
	let [x1, y1] = center1
	let [x2, y2] = center2

	let rect_width = Math.abs(y2 - y1)
	let rect_height = Math.abs(x2 - x1)

	let dx = x2 - x1
	let dy = y2 - y1

	if (dx * dy < 0) {
		// when `dx * dy < 0` one coordinate increases, other decreases, so `dx` and `dy` has different signs
		// top left -> bottom right
		x1 = 0
		y1 = 0
		x2 = rect_width
		y2 = rect_height
	} else {
		// bottom left -> top right
		x1 = 0
		y1 = rect_height
		x2 = rect_width
		y2 = 0
	}

	let e = document.createElementNS(SVG_NS, "svg")
	e.setAttribute("xmlns", SVG_NS)
	e.setAttribute("viewBox", `0 0 ${rect_width} ${rect_height}`)

	e.innerHTML = `<line id="${id}" x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" class="arrow" stroke-width="${STROKE}" />`
	return e
}

/**
 * Add padding when bounds forms too thin rectangle
 * @param  {number[][]} bounds
 * @return {import('leaflet').LatLngBoundsExpression}
 */
export function expand_thin_bounds(bounds) {
	const PAD = STROKE * 2

	let [[x1, y1], [x2, y2]] = bounds
	if (Math.abs(x2 - x1) < PAD) {
		if (x1 < x2) {
			x1 -= PAD
			x2 += PAD
		} else {
			x1 += PAD
			x2 -= PAD
		}
	} else if (Math.abs(y2 - y1) < PAD) {
		if (y1 < y2) {
			y1 -= PAD
			y2 += PAD
		} else {
			y1 += PAD
			y2 -= PAD
		}
	}

	return [[x1, y1], [x2, y2]]
}
