import { requestJson } from '../utils/Request';

export const LoginEpic = ({username, password}) => {
	return requestJson('/login');
};

export const LogoutEpic = ({username, password}) => {
	return requestJson('/logout');
};