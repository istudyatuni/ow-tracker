const SCALE = 0.95

const TEXT_HEIGHT = 100 * SCALE
const FONT_SIZE_EM = 1.8 * SCALE

const IMAGE_WIDTH = 200 * SCALE
const IMAGE_HEIGHT = IMAGE_WIDTH

export const CARD_HEIGHT = IMAGE_HEIGHT + TEXT_HEIGHT
export const CARD_WIDTH = IMAGE_WIDTH
const CARD_MARGIN = 2 * SCALE

const FULL_CARD_WIDTH = CARD_WIDTH + CARD_MARGIN * 2
const FULL_CARD_HEIGHT = CARD_HEIGHT + CARD_MARGIN

export const SVG_NS = "http://www.w3.org/2000/svg"

// tabler:medical-cross-circle
const STAR = '<path fill="none" class="explore-star" transform="translate(250, 0) scale(2, 2)" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0-18 0m9-4v8m3.5-6l-7 4m7 0l-7-4"/>'
export const STAR_SIZE = 50

const QUESTION = {
	x: IMAGE_WIDTH / 3,
	y: TEXT_HEIGHT + IMAGE_HEIGHT / 1.4,
	font: IMAGE_HEIGHT * 2 / 3,
}

/**
 * @param  {string} id Unique id, used for detecting clicked element
 * @param  {string} text
 * @param  {string} image_url
 * @param  {boolean} has_unexplored
 * @param  {string} category_class
 * @return {SVGElement}
 */
export function make_card_svg(id, text, image_url, has_unexplored, category_class) {
	let left_shift = 0
	let svg_width = FULL_CARD_WIDTH
	// increase card width for star
	// increase on both sides to not shift card on map (because of actual card's center change)
	if (image_url !== null && has_unexplored) {
		left_shift = STAR_SIZE
		svg_width += STAR_SIZE + left_shift
	}

	let e = document.createElementNS(SVG_NS, "svg")
	e.setAttribute("xmlns", SVG_NS)
	e.setAttribute("viewBox", `0 0 ${svg_width} ${FULL_CARD_HEIGHT}`)

	let star = ''
	let img_size = `x="${CARD_MARGIN + left_shift}" y="${TEXT_HEIGHT}" width="${IMAGE_WIDTH}" height="${IMAGE_HEIGHT}"`
	// draw question sign always so it's simple to toggle image on/off with css
	let img = `<rect ${img_size} class="img-q-bg" />
		<text x="${QUESTION.x + left_shift}" y="${QUESTION.y}" class="img-q-icon" style="font-size: ${QUESTION.font}px">?</text>`
	if (image_url !== null) {
		img += `<image href="${image_url}" ${img_size} class="spoiler" />`

		if (has_unexplored) {
			star = STAR
		}
	}

	// foreignObject is used to use <p> to have text auto-wrap
	e.innerHTML = `
		<rect x="${left_shift}" y="0" id="${id}" width="${FULL_CARD_WIDTH}" height="${FULL_CARD_HEIGHT}" class="card ${category_class}" />
		<switch>
			<foreignObject x="${left_shift}" y="0" width="${CARD_WIDTH}" height="${TEXT_HEIGHT}">
				<div xmlns="http://www.w3.org/1999/xhtml" class="card-text-wrapper">
					<p class="card-text mono spoiler" style="font-size: ${FONT_SIZE_EM}em">${text}</p>
				</div>
			</foreignObject>
			<text x="${left_shift}" y="0" font-size="20" text-anchor="middle" fill="white">svg viewer doesn't support html</text>
		</switch>
		${star}
		${img}`
	return e
}
