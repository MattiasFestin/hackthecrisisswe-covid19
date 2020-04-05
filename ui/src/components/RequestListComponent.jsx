import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import MaterialTable from 'material-table';
import { TransactionTypesEnum, TransactionTypesIcons, ConstraintTypeOpRender, TransactionDirIcons, TransactionDirEnum } from '../models/references';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'

import { useObservable } from 'react-use';

const useStyles = makeStyles((theme) => ({
  root: {
	flexGrow: 1,
	height: '50%',
	backgroundColor: 'hotpink'
  },
  table: {
	  height: '100%',
	  display: 'block'
  }
}));

export const RequestListComponent = (props) => {
	const classes = useStyles();
	const data = useObservable(props.transactionService.get$, []);

	return (
		<div className={classes.root + " ListComponent"}>
			<MaterialTable
				style={{
					height: '100%'
				}}
				
				columns={[
					{
						title: 'Type', field: 'transaction_direction_id', render: (row) =>  {
							return <span>
								<FontAwesomeIcon icon={TransactionDirIcons[row.transaction_direction_id]} /> &nbsp;
								{TransactionDirEnum[row.transaction_direction_id || 0]}
							</span>
						}
					},
					{ title: 'What', field: 'transaction_type_id', render: (row) =>  {
						return <span>
							<FontAwesomeIcon icon={TransactionTypesIcons[row.transaction_type_id]} /> &nbsp;
							{TransactionTypesEnum[row.transaction_type_id]}
						</span>
					}},
					{ title: 'Description', field: 'what' },
					{ title: 'Constraints', editable: 'never', render: (row) =>  {
						if (!row.constraints) {
							return;
						}
						return <ul>
							{row.constraints.map((c, index) => <li key={index}>{ConstraintTypeOpRender[c.op]} {c.value} {c.unit}</li>)}
						</ul>;
					}},
					{ title: 'Where', field: 'where', editable: 'never', render: (row) =>  {
						if (!row.where) return '';
						return <span>
							{row.where.road} ({row.where.neighbourhood}, {row.where.suburb})<br/>
							{row.where.town || row.where.city}, {row.where.state}
						</span>;
					}},
					{ title: 'Priority', field: 'priority' }
				]}
				data={data}
				title="Transactions"
				options={{
					filtering: true,
					exportButton: true,
					pageSize: 1000,
				}}
				editable={{
					// onRowAdd: newData => new Promise((resolve, reject) => {
					// 	// setTimeout(() => {
					// 	//   {
					// 	// 	const data = this.state.data;
					// 	// 	data.push(newData);
					// 	// 	this.setState({ data }, () => resolve());
					// 	//   }
					// 	//   resolve()
					// 	// }, 1000)
					// 	debugger;
					// 	props.transactionService.exec({
					// 		action: 'insert',
					// 		data: newData
					// 	});
					// }),
					onRowUpdate: (newData, oldData) => new Promise((resolve, reject) => {
						// setTimeout(() => {
						//   {
						// 	const data = this.state.data;
						// 	const index = data.indexOf(oldData);
						// 	data[index] = newData;
						// 	this.setState({ data }, () => resolve());
						//   }
						//   resolve()
						// }, 1000)
					}),
					onRowDelete: oldData => new Promise((resolve, reject) => {
					// 	setTimeout(() => {
					// 	  {
					// 		let data = this.state.data;
					// 		const index = data.indexOf(oldData);
					// 		data.splice(index, 1);
					// 		this.setState({ data }, () => resolve());
					// 	  }
					// 	  resolve()
					// 	}, 1000)
					}),
				}}
			/>
		</div>
	);
}