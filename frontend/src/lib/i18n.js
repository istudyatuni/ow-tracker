import { derived } from "svelte/store";
import { FluentBundle, FluentResource } from "@fluent/bundle";

import { set_tr_bundle, tr_bundle } from "@/lib/stores";
import { detect_language, language_to_code, LANGUAGES } from "@/lib/language";

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
		console.error("error during i18n inittialization", errors);
	}

	set_tr_bundle(bundle);
}

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

export const t = derived(tr_bundle, translator_fn);
