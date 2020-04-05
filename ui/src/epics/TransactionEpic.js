import { requestJson } from '../utils/Request';

export const transactionListEpic = () => {
	return requestJson('/transactions');
};


export const transactionGetEpic = (id) => {
	return requestJson(`/transaction/${id}`);
};

export const transactionCreate = (data) => {
	return requestJson(`/transaction`, {
		method: 'PUT',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(data)
	});
};