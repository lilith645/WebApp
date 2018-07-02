import React from 'react';
import './index.css';
import PublicPages from './public.js';

class App extends React.Component {
  constructor(props) {
    super(props);
  }
  
  render() {
    return (
      <div>
        <PublicPages/>
      </div>
    )
  }
}

export default App;
