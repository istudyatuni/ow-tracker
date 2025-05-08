// https://github.com/sveltejs/svelte/issues/14929#issuecomment-2576621579
export function renderSnippet(snippet) {
	let target = document.createElement("span");
	let anchor = document.createComment("");
	target.appendChild(anchor);
	snippet(anchor);
	anchor.remove();
	return target.innerHTML;
}
