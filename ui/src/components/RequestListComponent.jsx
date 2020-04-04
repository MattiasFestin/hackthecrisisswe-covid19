import React from 'react';
import { makeStyles } from '@material-ui/core/styles';
import MaterialTable from 'material-table';
import { TransactionTypesEnum, TransactionTypesIcons, ConstraintTypeOpRender } from '../models/references';
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
	const data = useObservable(props.data, []);

	return (
		<div className={classes.root + " ListComponent"}>
			<MaterialTable
				style={{
					height: '100%'
				}}
				columns={[
					{ title: 'Type', field: 'transaction_type_id', render: (row) =>  {
						return <span>
							<FontAwesomeIcon icon={TransactionTypesIcons[row.transaction_type_id]} /> &nbsp;
							{TransactionTypesEnum[row.transaction_type_id]}
						</span>
					}},
					{ title: 'What', field: 'what' },
					{ title: 'Constraints', render: (row) =>  {
						debugger;
						if (!row.constraints) {
							return;
						}
						return <ul>
							{row.constraints.map(c => <li>{ConstraintTypeOpRender[c.op]} {c.value} {c.unit}</li>)}
						</ul>;
					}},
					{ title: 'Where', field: 'where' },
					{ title: 'Priority', field: 'priority' }
				]}
				data={data}
				title="Requests"
				options={{
					filtering: true
				}}
			/>
		</div>
	);
}