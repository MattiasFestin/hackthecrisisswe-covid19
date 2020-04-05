import { Subject, Observable } from 'rxjs';
import { transactionListEpic, transactionCreate } from '../epics/TransactionEpic';
import { filter } from 'rxjs/operators';

// const get$ = Subject.create({
// 	next: (data) => data
// });
const exec = new Subject();
export const transactionService = {
	get$: Observable.create( observer => {
		exec
			.pipe(filter(({action}) => action === 'getList'))
			.subscribe(() => {
				transactionListEpic().subscribe(
					(data) => {
						console.table(data);
						observer.next(data);
					},
					err => observer.error(err),
					() => observer.complete()
				);
			});
	}),
	exec
};


exec
	.pipe(filter(({action}) => action === 'insert'))
	.subscribe(({data}) => {
		transactionCreate(data).subscribe(
			(data) => {
				window.location.reload();
			},
			err => console.error(err)
		);
	});