import type { Readable } from "svelte/store";

interface WritableKV<T> extends Readable<T> {
	set(this: void, key: string, value: any): void;
	delete(this: void, key: string): void;
}

type VersionedStore = {
	version: number;
};

type SettingsStore = VersionedStore & {
	selected_categories_version: number;
	/** Hide all spoilers */
	hide_spoilers: boolean;
	/** Separate field for hiding `stranger` category when showing full map */
	hide_dlc: boolean;
	consider_ignored_facts: boolean;
	show_ignored_facts: boolean;
};

type SessionSettingsStore = VersionedStore & {
	welcome_popup_done: boolean;
};
