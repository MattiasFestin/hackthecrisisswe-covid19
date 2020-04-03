import { from } from 'rxjs';
import { map } from 'rxjs/operators';

export const requestJson = (input, init) => {
	return request(input, init).pipe(map(x => x.json()));
};
export const requestText = (input, init) => {
	return request(input, init).pipe(map(x => x.text()));
};
export const request = (input, init) => {
	return from(fetch(window.BASE_URL + input, init));
};