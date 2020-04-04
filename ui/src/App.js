import React from 'react';
import logo from './logo.svg';
import './App.css';

import { createMuiTheme, ThemeProvider } from '@material-ui/core/styles';

import { AppBarComponent } from './components/AppBarComponent';
import { RequestListComponent } from './components/RequestListComponent';
import { MapComponent } from './components/MapComponent';

import { library } from '@fortawesome/fontawesome-svg-core'
import { fas } from '@fortawesome/free-solid-svg-icons'

library.add(fas)

function App() {
  const theme = createMuiTheme({
    //TODO:
  });

  const data = [
    { 
      transaction_type_id: 1,
      transaction_direction_id: 1,
      constraints: [{
        unit: 'nbr',
        op: 'numeric_leq',
        value: 100
      }],
      what: 'Protective gear',
      where: 'Karolinska Universitetssjukhuset Solna, Stockholm',
      coordinate: [59.3486777, 18.0304581],
      priority: 100
    },
    { 
      transaction_type_id: 1,
      transaction_direction_id: 1,
      constraints: [{
        unit: 'nbr',
        op: 'numeric_gt',
        value: 1
      }, {
        unit: 'nbr',
        op: 'numeric_lt',
        value: 100
      }],
      what: 'Protective gear',
      where: 'Capio Sankt Görans Sjukhus, Stockholm',
      coordinate: [59.334105, 18.020388],
      priority: 50
    }
  ];

  return (
    <React.StrictMode>
      <ThemeProvider theme={theme}>
        <AppBarComponent />
        <MapComponent  data={data} />
        <RequestListComponent data={data} />
        {/* <div className="App">
          <header className="App-header">
            <img src={logo} className="App-logo" alt="logo" />
            <p>
              Edit <code>src/App.js</code> and save to reload.
            </p>
            <a
              className="App-link"
              href="https://reactjs.org"
              target="_blank"
              rel="noopener noreferrer"
            >
              Learn React
            </a>
          </header>
        </div> */}
      </ThemeProvider>
    </React.StrictMode>
  );
}

export default App;
