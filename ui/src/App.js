import React from 'react';
// import logo from './logo.svg';
import './App.css';

import { createMuiTheme, ThemeProvider } from '@material-ui/core/styles';

import { library } from '@fortawesome/fontawesome-svg-core'
import { fas } from '@fortawesome/free-solid-svg-icons'


// import { AppBarComponent } from './components/AppBarComponent';
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

  return (
    <React.StrictMode>
      <ThemeProvider theme={theme}>
        <MapComponent  transactionService={transactionService} komuner={komuner} regioner={regioner} />
        <RequestListComponent transactionService={transactionService} />
      </ThemeProvider>
    </React.StrictMode>
  );
};