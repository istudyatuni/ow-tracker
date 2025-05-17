export type VersionedStore = {
	version: number;
};

export type SettingsStore = VersionedStore & {
	selected_categories_version: number;
	/** Hide all spoilers */
	hide_spoilers: boolean;
	/** Separate field for hiding `stranger` category when showing full map */
	hide_dlc: boolean;
	consider_ignored_facts: boolean;
	show_ignored_facts: boolean;
};

export type SessionSettingsStore = VersionedStore & {
	welcome_popup_done: boolean;
};
