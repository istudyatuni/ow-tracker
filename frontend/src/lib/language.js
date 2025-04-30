const LANGUAGES = {
	English: 'english',
	SpanishLa: 'spanish_la',
	German: 'german',
	French: 'french',
	Italian: 'italian',
	Polish: 'polish',
	PortugueseBr: 'portuguese_br',
	Japanese: 'japanese',
	Russian: 'russian',
	ChineseSimple: 'chinese_simple',
	Korean: 'korean',
	Turkish: 'turkish',
}

export function detect_language() {
	let lang = null
	for (let code of navigator.languages) {
		lang = code_to_lang(code.split('-')[0])
		if (lang !== null) {
			return lang
		}
	}
	return LANGUAGES.English
}

function code_to_lang(code) {
	switch (code) {
		case 'en': return LANGUAGES.English
		case 'es': return LANGUAGES.SpanishLa
		case 'de': return LANGUAGES.German
		case 'fr': return LANGUAGES.French
		case 'it': return LANGUAGES.Italian
		case 'pl': return LANGUAGES.Polish
		case 'pt': return LANGUAGES.PortugueseBr
		case 'ja': return LANGUAGES.Japanese
		case 'ru': return LANGUAGES.Russian
		case 'zh': return LANGUAGES.ChineseSimple
		case 'ko': return LANGUAGES.Korean
		case 'tr': return LANGUAGES.Turkish
		default: return null
	}
}
