import React, { useState, useEffect } from 'react';
import * as wasm from './wasm/wasm.js';

function App() {
  const [items, setItems] = useState([]);

  useEffect(() => {
    setItems(wasm.get_items());
  }, []);

  function addItem() {
    const item = document.getElementById('newItem').value;
    wasm.add_item(item);
    setItems(wasm.get_items());
    document.getElementById('newItem').value = '';
  }

  return (
    <div className="App">
      <h1>Grocery List</h1>
      <ul>
        {items.map((item, index) => (
          <li key={index}>{item}</li>
        ))}
      </ul>
      <input type="text" id="newItem" placeholder="Add Item" />
      <button onClick={addItem}>Add</button>
    </div>
  );
}

export default App;