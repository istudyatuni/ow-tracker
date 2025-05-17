import { derived, writable } from "svelte/store";
import { FluentBundle, FluentResource } from "@fluent/bundle";

import { detect_language, language_to_code, LANGUAGES } from "@/lib/language";

/** @type {import("svelte/store").Writable<import("@fluent/bundle").FluentBundle> | null} */
const tr_bundle = writable(null);

export async function init_i18n() {
	let lang = detect_language();
	let code = language_to_code(lang);
	if (code === null) {
		lang = LANGUAGES.English;
		code = "en";
	}
	let translations = await (
		await fetch(`${import.meta.env.BASE_URL}/translations/ui/${lang}.ftl`)
	).text();

	let res = new FluentResource(translations);
	let bundle = new FluentBundle(code);

	let errors = bundle.addResource(res);
	if (errors.length) {
		console.error("error during i18n initialization", errors);
	}

	tr_bundle.set(bundle);
}

/**
 * @param  {FluentBundle} bundle
 * @return {(id: string, args?: Object<string, any>) => string}
 */
function translator_fn(bundle) {
	return (id, args = {}) => {
		if (bundle === null) {
			return "";
		}

		let msg = bundle.getMessage(id);
		if (msg?.value) {
			return bundle.formatPattern(msg.value, args);
		}
		console.warn("no value for message with id", id);
		return id;
	};
}

/**
 * Storage for current translation
 *
 * Usage:
 *
 * ```js
 * // in svelte component
 * {$t('translation-key')}
 *
 * // or with arguments
 * {$t('translation-key', { argument: 'value' })}
 * ```
 */
export const t = derived(tr_bundle, translator_fn);
