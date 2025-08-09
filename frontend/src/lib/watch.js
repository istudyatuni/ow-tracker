import { get } from "svelte/store";
import { PROFILE_WATCH_UPDATES } from "./stores";

export function listen_profile_update(id) {
	let url = import.meta.env.VITE_SERVER + "/api/watch?id=" + id;
	const ev = new EventSource(url);
	ev.onmessage = (event) => {
		if (event.data == "save-updated") {
			console.log("got event:", event);
			if (get(PROFILE_WATCH_UPDATES)) {
				window.location.reload();
			}
		}
	};
}
