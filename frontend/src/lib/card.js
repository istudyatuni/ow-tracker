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

const QUESTION = {
	x: IMAGE_WIDTH / 3,
	y: TEXT_HEIGHT + IMAGE_HEIGHT / 1.4,
	font: IMAGE_HEIGHT * 2 / 3,
}

/**
 * @param  {string} id Unique id, used for detecting clicked element
 * @param  {string} text
 * @param  {string} image_url
 * @param  {string} color
 * @param  {string} hover_color
 * @return {SVGElement}
 */
export function make_card_svg(id, text, image_url, color, hover_color) {
	let e = document.createElementNS(SVG_NS, "svg")
	e.setAttribute("xmlns", SVG_NS)
	e.setAttribute("viewBox", `0 0 ${FULL_CARD_WIDTH} ${FULL_CARD_HEIGHT}`)

	// hack to have correct hover colors
	let hover_class = hover_color.replace('#', 'c')

	let img
	let img_size = `x="${CARD_MARGIN}" y="${TEXT_HEIGHT}" width="${IMAGE_WIDTH}" height="${IMAGE_HEIGHT}"`
	if (image_url === null) {
		img = `<rect ${img_size} class="img-q-bg" />
			<text x="${QUESTION.x}" y="${QUESTION.y}" class="img-q-icon" style="font-size: ${QUESTION.font}px">?</text>`
	} else {
		img = `<image href="${image_url}" ${img_size} />`
	}

	// foreignObject is used to use <p> to have text auto-wrap
	e.innerHTML = `
		<rect x="0" y="0" id="${id}" width="${FULL_CARD_WIDTH}" height="${FULL_CARD_HEIGHT}" fill="${color}" class="card ${hover_class}" />
		<switch>
			<foreignObject x="0" y="0" width="${CARD_WIDTH}" height="${TEXT_HEIGHT}">
				<div xmlns="http://www.w3.org/1999/xhtml" class="card-text-wrapper">
					<p class="card-text mono spoiler" style="font-size: ${FONT_SIZE_EM}em">${text}</p>
				</div>
			</foreignObject>
			<text x="0" y="0" font-size="20" text-anchor="middle" fill="white">svg viewer doesn't support html</text>
		</switch>
		${img}`
	return e
}
