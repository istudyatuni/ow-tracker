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

export const LANGUAGE_NAMES = {
	english: 'English',
	spanish_la: 'Español',
	german: 'Deutsch',
	french: 'Français',
	italian: 'Italiano',
	polish: 'Polski',
	portuguese_br: 'Português',
	japanese: '日本語',
	russian: 'Русский',
	chinese_simple: '简化字',
	korean: '한국어',
	turkish: 'Türkçe',
}

export function detect_language() {
	return get_language()
		|| code_to_lang(navigator.languages.find((code) => code_to_lang(code.split('-')[0])))
		|| LANGUAGES.English
}

export function save_language(id) {
	localStorage.setItem('language', id)
}

export function get_language() {
	return localStorage.getItem('language')
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
