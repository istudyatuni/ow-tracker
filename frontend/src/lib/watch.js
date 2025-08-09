export function listen_profile_update(id) {
	let now = Date.now();

	let url = import.meta.env.VITE_SERVER + "/api/watch?id=" + id;
	const ev = new EventSource(url);
	ev.onmessage = (event) => {
		if (Date.now() - now < 3000) {
			console.log("skipping reload because just reloaded already");
			return;
		}
		if (event.data == "save-updated") {
			console.log("got event:", event);
			window.location.reload();
		}
	};
}
