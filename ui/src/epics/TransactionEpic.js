import { requestJson } from '../utils/Request';

export const LoginEpic = ({username, password}) => {
	return requestJson('/transactions');
};