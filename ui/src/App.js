import React from 'react';
import logo from './logo.svg';
import './App.css';

import { createMuiTheme, ThemeProvider } from '@material-ui/core/styles';

import { library } from '@fortawesome/fontawesome-svg-core'
import { fas } from '@fortawesome/free-solid-svg-icons'


import { AppBarComponent } from './components/AppBarComponent';
import { RequestListComponent } from './components/RequestListComponent';
import { MapComponent } from './components/MapComponent';

// import { transactionListEpic } from './service/transactionService';
import { transactionService } from './service/transactionService';

library.add(fas)

const komuner = require('./data/komun.json');
const regioner = require('./data/fa_region.json');


export default function App() {
  const theme = createMuiTheme({
    //TODO:
  });
  setTimeout(() => {
    transactionService.exec.next({action: 'getList'});
  }, 1000);
  // transactionService.get$.next(['test'])
  // console.log(transactionService);
  // debugger;
  // const data$ = transactionListEpic();
  // const data = await ;

  // const data = [
  //   { 
  //     transaction_type_id: 1,
  //     transaction_direction_id: 1,
  //     constraints: [{
  //       unit: 'nbr',
  //       op: 'numeric_leq',
  //       value: 100
  //     }],
  //     what: 'Protective gear',
  //     where: 'Karolinska Universitetssjukhuset Solna, Stockholm',
  //     coordinate: [59.3486777, 18.0304581],
  //     priority: 100
  //   },
  //   { 
  //     transaction_type_id: 1,
  //     transaction_direction_id: 1,
  //     constraints: [{
  //       unit: 'nbr',
  //       op: 'numeric_gt',
  //       value: 1
  //     }, {
  //       unit: 'nbr',
  //       op: 'numeric_lt',
  //       value: 100
  //     }],
  //     what: 'Protective gear',
  //     where: 'Capio Sankt Görans Sjukhus, Stockholm',
  //     coordinate: [59.334105, 18.020388],
  //     priority: 50
  //   }
  // ];
/* <React.StrictMode> */
  return (
    <ThemeProvider theme={theme}>
      {/* <AppBarComponent /> */}
      <MapComponent  transactionService={transactionService} komuner={komuner} regioner={regioner} />
      <RequestListComponent transactionService={transactionService} />
    </ThemeProvider>
  );
};