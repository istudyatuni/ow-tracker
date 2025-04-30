const TEXT_HEIGHT = 100

const IMAGE_WIDTH = 200
const IMAGE_HEIGHT = 200

export const CARD_HEIGHT = IMAGE_HEIGHT + TEXT_HEIGHT
export const CARD_WIDTH = 200
const CARD_MARGIN = 2

const FULL_CARD_WIDTH = CARD_WIDTH + CARD_MARGIN * 2
const FULL_CARD_HEIGHT = CARD_HEIGHT + CARD_MARGIN

// todo: vertical-align not work
const TEXT_STYLE = 'margin: auto; font-family: ui-sans-serif, system-ui, sans-serif; text-align: center; vertical-align: middle;'

/**
 * @param  {string} text
 * @param  {string} image_url
 * @param  {string} color
 * @return {string}
 */
export function make_card_svg(text, image_url, color) {
  return `<svg width="${FULL_CARD_WIDTH}" height="${FULL_CARD_HEIGHT}" xmlns="http://www.w3.org/2000/svg">
    <rect x="0" y="0" width="${FULL_CARD_WIDTH}" height="${FULL_CARD_HEIGHT}" fill="${color}" />
    <switch>
      <foreignObject x="0" y="0" width="${CARD_WIDTH}" height="${TEXT_HEIGHT}">
        <p xmlns="http://www.w3.org/1999/xhtml" style="font-size: 1.6em; ${TEXT_STYLE}">${text}</p>
      </foreignObject>
      <text x="0" y="0" font-size="20" text-anchor="middle" fill="white">svg viewer doesn't support html</text>
    </switch>
    <image href="${image_url}" x="${CARD_MARGIN}" y="${TEXT_HEIGHT}" width="${IMAGE_WIDTH}" height="${IMAGE_HEIGHT}" />
  </svg>`
}
