export const CATEGORIES = [
	"orbital-canon",
	"quantum-moon",
	"vessel",
	"ash-twin-project",
	"stranger",
	"nomai-fate",
	"other",
];

export const CURIOSITY = {
	COMET_CORE: 'COMET_CORE',
	INVISIBLE_PLANET: 'INVISIBLE_PLANET',
	QUANTUM_MOON: 'QUANTUM_MOON',
	SUNKEN_MODULE: 'SUNKEN_MODULE',
	TIME_LOOP: 'TIME_LOOP',
	VESSEL: 'VESSEL',

	// custom, used for cards without explicit curiosity
	OTHER: 'OTHER',
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
		case "nomai-fate": return CURIOSITY.COMET_CORE
		case "other": return CURIOSITY.OTHER
	}
}

export function curiosity_to_category(c) {
	switch (c) {
		case CURIOSITY.SUNKEN_MODULE: return "orbital-canon"
		case CURIOSITY.QUANTUM_MOON: return "quantum-moon"
		case CURIOSITY.VESSEL: return "vessel"
		case CURIOSITY.TIME_LOOP: return "ash-twin-project"
		case CURIOSITY.INVISIBLE_PLANET: return "stranger"
		case CURIOSITY.COMET_CORE: return "nomai-fate"
	}
	return "other"
}

/**
 * If should show card/arrow with defined curiosity
 * @param  {Set.<string>}     hide_curiosities Which curiosities to hide
 * @param  {string|undefined} curiosity        Curiosity of card
 * @return {boolean}
 */
export function should_hide_curiosity(hide_curiosities, curiosity) {
	return hide_curiosities.has(curiosity)
}
