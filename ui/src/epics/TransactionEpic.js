import { requestJson } from '../utils/Request';

export const transactionListEpic = () => {
	return requestJson('/transactions');
};