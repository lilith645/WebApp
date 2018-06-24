import React from 'react';
import './index.css';

class LoginManager extends React.Component {
  constructor(props) {
    super(props);
    this.renderCode = this.renderCode.bind(this);
    this.renderDb = this.renderDb.bind(this);
    this.state = {
      code: '5217',
      dbinfo: 'Place holder',
    };
  }
  
  renderInputBox() {
    return (
      <p>
      Username <input type="text" name="username" placeholder="Username..."></input>
      <br></br>
      <br></br>
      Password <input type="text" name="password" placeholder="Password..."></input>
      <br></br>
      <br></br>
      </p>
    )
  }
  
  renderCode() {
    let stuff = this.state.code;
    
    return (
      <p>
      {stuff}
      <button type="submit" onClick={() => this.handleGetCode()}>Get New Code</button>
      </p>
    )
  }
  
  renderDb() {
    let db = this.state.dbinfo;
    
    return (
      <p>
      {db}
      <button type="submit" onClick={() => this.handleGetDbInfo()}>Update Db Info</button>
      </p>
    )
  }
  
  handleGetCode() {
    fetch("http://localhost:8000/api").then(r => r.text()).then(code => {
      this.setState({
        code: code
      });
    });
  }
  
  handleGetDbInfo() {
    fetch("http://localhost:8000/db").then(r => r.text()).then(query => {
      this.setState({
        dbinfo: query
      });
    });
  }
  
  render(props) {
    let inputBox;
    this.props.isLoggedIn ? (
      inputBox = "Already logged in"
    ) : (
      inputBox = <this.renderInputBox />
    );
    
    let code = <this.renderCode />;
    let db = <this.renderDb />;
    
    return (
      <div className="Center">
        {inputBox}
        <button type="submit" onClick={() => this.props.onClick()}>Log in</button>
        {code}
        {db}
      </div>
    )
  }
}

export default LoginManager;
