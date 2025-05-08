/**
 * @param  {Blob} blob
 * @return {Promise}
 */
export async function to_data_url(blob) {
	// https://stackoverflow.com/a/18650249
	return new Promise((resolve, _) => {
		const reader = new FileReader();
		reader.onloadend = () => resolve(reader.result);
		reader.readAsDataURL(blob);
	});
}
