import React from 'react';
import './index.css';
import LoginManager from './LoginManager.js';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.handleClick = this.handleClick.bind(this);
    this.state = {
      isLoggedIn: false,
    };
  }
  
  handleClick() {
    this.setState({
      isLoggedIn: true,
    })
  }
  
  render() {
    const stuff = this.state.isLoggedIn;
    
    let content;
    
    if (stuff) {
      content = <Info />;
    } else {
      content = <LoginManager onClick={this.handleClick} loggedIn={this.state.isLoggedIn} />;
    }
    
    return (
      <div>
        {content}
      </div>
    )
  }
}

class Info extends React.Component {
  render() {
    return (
      <div>
        <p className="Center">
          You are logged in!
        </p>
      </div>
    )
  }
}

export default App;
