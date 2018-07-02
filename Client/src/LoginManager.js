import React from 'react';
import './index.css';

class LoginManager extends React.Component {
  constructor(props) {
    super(props);
    this.renderCode = this.renderCode.bind(this);
    this.renderDb = this.renderDb.bind(this);
    this.renderLoginBox = this.renderLoginBox.bind(this);
    this.handleUsernameChange = this.handleUsernameChange.bind(this);
    this.handlePasswordChange = this.handlePasswordChange.bind(this);
    this.handleLoginClick = this.handleLoginClick.bind(this);
    this.state = {
      loggedin: false,
      code: '5217',
      dbinfo: 'Place holder',
      username: '',
      email: 'Test@email.com',
      password: ''
    };
  }
  
  renderLoginBox() {
    return (
      <p>
        Username: <input id= "username" type="text" name="username" placeholder="Username..." value={this.state.username} onChange={this.handleUsernameChange}></input><br></br>
        Password: <input id="password" type="password" name="password" placeholder="Password..." value={this.state.password} onChange={this.handlePasswordChange}></input><br></br>
      <button type="submit" onClick={this.handleLoginClick}>Log in</button>
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
  
  handleUsernameChange(e) {
    this.setState({
      username: e.target.value
    });
  }
  
  handlePasswordChange(e) {
    this.setState({
      password: e.target.value
    });
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
  
  handleLoginClick() {
    var data = new FormData();
    data.append("user", JSON.stringify({ username: this.state.username, email: this.state.email, password: this.state.password}));
    
    fetch("http://localhost:8000/users", {
      method: "POST",
      mode: 'no-cors',
      headers: {
        'Accept': 'application/json, text/plain, */*',
        'Content-Type': 'application/json'
      },
      body: data
/*      headers: {
        'Accept': 'application/json',
        'Content-Type':'application/json'
      },
       body: JSON.stringify({"user": {
          "username" : this.state.username,
          "email"    : "temp@email.com",
          "password" : this.state.password,
        }
       })*/
    }).then(data => data)
    .then(data => console.log(data));
    
    this.setState({
      isLoggedIn: true,
    })
  }
  
  render(props) {
    let loginBox;
    this.props.isLoggedIn ? (
      loginBox = "Already logged in"
    ) : (
      loginBox = <this.renderLoginBox />
    );
    
    let code = <this.renderCode />;
    let db = <this.renderDb />;
    
    return (
      <div className="Center">
        {loginBox}
        {code}
        {db}
      </div>
    )
  }
}

export default LoginManager;
