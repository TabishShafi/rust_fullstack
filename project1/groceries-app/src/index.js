import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';

import('./wasm/wasm.js')
  .then(async (wasm) => {
    await wasm.default(); // Initialize Wasm module
    ReactDOM.render(
      <React.StrictMode>
        <App />
      </React.StrictMode>,
      document.getElementById('root')
    );
  })
  .catch(console.error);
