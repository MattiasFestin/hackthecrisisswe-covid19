import { from } from 'rxjs';

export const requestJson = (input, init) => {
	return from(
		fetch(window.BASE_URL + input, init)
		.then(x => x.json())
	);
};
export const requestText = (input, init) => {
	return from(
		fetch(window.BASE_URL + input, init)
		.then(x => x.text())
	);
};