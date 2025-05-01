import { SVG_NS } from './card'

/**
 * Make svg arrow from center1 to center2
 * @param  {string} id
 * @param  {import('leaflet').LatLngTuple} center1
 * @param  {import('leaflet').LatLngTuple} center2
 * @return {SVGElement}
 */
export function make_rumor_arrow(id, center1, center2, color = 'gray') {
	let [x1, y1] = center1
	let [x2, y2] = center2

	let rect_width = Math.max(y1, y2) - Math.min(y1, y2)
	let rect_height = Math.max(x1, x2) - Math.min(x1, x2)

	let dx = x2 - x1
	let dy = y2 - y1

	// one coordinate increases, other decreases
	if (dx * dy < 0) {
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
	// todo: fix thin lines when rect width or height < stroke width
	e.setAttribute("viewBox", `0 0 ${rect_width} ${rect_height}`)

	e.innerHTML = `<style>line{pointer-events:auto}</style><line id="${id}" x1="${x1}" y1="${y1}" x2="${x2}" y2="${y2}" stroke="${color}" stroke-width="20" />`

	return e
}
