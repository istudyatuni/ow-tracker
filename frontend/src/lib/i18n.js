import { FluentBundle, FluentResource } from "@fluent/bundle";

import { set_tr_bundle, translator } from "@/lib/stores";
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

export { translator as t };
