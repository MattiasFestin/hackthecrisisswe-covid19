import { from } from 'rxjs';

export const requestJson = (input, init, BASE_URL) => {
	BASE_URL = BASE_URL || window.BASE_URL;
	return from(
		fetch(BASE_URL + input, init)
		.then(x => x.json())
	);
};
export const requestText = (input, init, BASE_URL) => {
	BASE_URL = BASE_URL || window.BASE_URL;
	return from(
		fetch(BASE_URL + input, init)
		.then(x => x.text())
	);
};