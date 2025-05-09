export const CATEGORY = {
	ORBITAL_CANON: "orbital-canon",
	QUANTUM_MOON: "quantum-moon",
	VESSEL: "vessel",
	ASH_TWIN_PROJECT: "ash-twin-project",
	STRANGER: "stranger",
	NOMAI_FATE: "nomai-fate",
	OTHER: "other",
};

export const CATEGORIES = Object.values(CATEGORY);

export const CURIOSITY = {
	COMET_CORE: "COMET_CORE",
	INVISIBLE_PLANET: "INVISIBLE_PLANET",
	QUANTUM_MOON: "QUANTUM_MOON",
	SUNKEN_MODULE: "SUNKEN_MODULE",
	TIME_LOOP: "TIME_LOOP",
	VESSEL: "VESSEL",

	// custom, used for cards without explicit curiosity
	OTHER: "OTHER",
};

export function default_enabled_categories() {
	return default_categories(true);
}

export function default_disabled_categories() {
	return default_categories(false);
}

export function default_categories(enabled) {
	return Object.fromEntries(CATEGORIES.map((c) => [c, enabled]));
}

export function curiosity_to_category(c) {
	switch (c) {
		case CURIOSITY.SUNKEN_MODULE:
			return CATEGORY.ORBITAL_CANON;
		case CURIOSITY.QUANTUM_MOON:
			return CATEGORY.QUANTUM_MOON;
		case CURIOSITY.VESSEL:
			return CATEGORY.VESSEL;
		case CURIOSITY.TIME_LOOP:
			return CATEGORY.ASH_TWIN_PROJECT;
		case CURIOSITY.INVISIBLE_PLANET:
			return CATEGORY.STRANGER;
		case CURIOSITY.COMET_CORE:
			return CATEGORY.NOMAI_FATE;
	}
	return CATEGORY.OTHER;
}
