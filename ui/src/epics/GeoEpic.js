import { requestJson } from '../utils/Request';

export const komunerEpic = () => {
	return requestJson('/komun.geojson', undefined, window.location.origin);
};