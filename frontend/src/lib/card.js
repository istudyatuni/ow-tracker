const SCALE = 0.85

const TEXT_HEIGHT = 100 * SCALE
const FONT_SIZE_EM = 1.6 * SCALE

const IMAGE_WIDTH = 200 * SCALE
const IMAGE_HEIGHT = IMAGE_WIDTH

export const CARD_HEIGHT = IMAGE_HEIGHT + TEXT_HEIGHT
export const CARD_WIDTH = IMAGE_WIDTH
const CARD_MARGIN = 2 * SCALE

const FULL_CARD_WIDTH = CARD_WIDTH + CARD_MARGIN * 2
const FULL_CARD_HEIGHT = CARD_HEIGHT + CARD_MARGIN

// todo: vertical-align not work
const TEXT_STYLE = 'margin: auto; font-family: ui-sans-serif, system-ui, sans-serif; text-align: center; vertical-align: middle; color: black;'

export const SVG_NS = "http://www.w3.org/2000/svg"

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
  // todo: probably it's possible to not embed it inside svg
  let hover_class = hover_color.replace('#', 'c')

  // foreignObject is used to use <p> to have text auto-wrap
  e.innerHTML = `<style>
      svg:hover > .${hover_class} {
        fill: ${hover_color};
      }
      rect {
        pointer-events:auto;
      }
    </style>
    <rect x="0" y="0" id="${id}" width="${FULL_CARD_WIDTH}" height="${FULL_CARD_HEIGHT}" fill="${color}" class="${hover_class}" />
    <switch>
      <foreignObject x="0" y="0" width="${CARD_WIDTH}" height="${TEXT_HEIGHT}">
        <p xmlns="http://www.w3.org/1999/xhtml" style="font-size: ${FONT_SIZE_EM}em; ${TEXT_STYLE}">${text}</p>
      </foreignObject>
      <text x="0" y="0" font-size="20" text-anchor="middle" fill="white">svg viewer doesn't support html</text>
    </switch>
    <image href="${image_url}" x="${CARD_MARGIN}" y="${TEXT_HEIGHT}" width="${IMAGE_WIDTH}" height="${IMAGE_HEIGHT}" />`
  return e
}
