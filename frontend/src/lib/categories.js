export const CATEGORIES = [
	"orbital-canon",
	"quantum-moon",
	"vessel",
	"ash-twin-project",
	"stranger",
	"other",
];

export const CURIOSITY = {
	COMET_CORE: 'COMET_CORE',
	INVISIBLE_PLANET: 'INVISIBLE_PLANET',
	QUANTUM_MOON: 'QUANTUM_MOON',
	SUNKEN_MODULE: 'SUNKEN_MODULE',
	TIME_LOOP: 'TIME_LOOP',
	VESSEL: 'VESSEL',
}

export function default_categories() {
	return Object.fromEntries(CATEGORIES.map((c) => [c, true]))
}

export function category_to_curiosity(c) {
	switch (c) {
		case "orbital-canon": return CURIOSITY.SUNKEN_MODULE
		case "quantum-moon": return CURIOSITY.QUANTUM_MOON
		case "vessel": return CURIOSITY.VESSEL
		case "ash-twin-project": return CURIOSITY.TIME_LOOP
		case "stranger": return CURIOSITY.INVISIBLE_PLANET
		case "other": return CURIOSITY.COMET_CORE
	}
}

/**
 * If should show card/arrow with defined curiosity
 * @param  {Set.<string>}         hide_curiosities Which curiosities to hide
 * @param  {string|undefined} curiosity        Curiosity of card
 * @return {boolean}
 */
export function should_show_curiosity(hide_curiosities, curiosity) {
	if (curiosity !== undefined) {
		return hide_curiosities.has(curiosity)
	}
	return hide_curiosities.has(CURIOSITY.COMET_CORE)
}
