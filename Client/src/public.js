import React from 'react';
import './index.css';
import './public.css';
import LoginManager from './LoginManager.js';

class PublicPages extends React.Component {
  constructor(props) {
    super(props);
    
    this.renderHeader = this.renderHeader.bind(this);
    this.handleHomeClick = this.handleHomeClick.bind(this);
    this.handleDetailsClick = this.handleDetailsClick.bind(this);
    this.handleContactClick = this.handleContactClick.bind(this);
    this.handleLoginClick = this.handleLoginClick.bind(this);
    
    this.state = {
      render_home: true,
      render_details: false,
      render_contact: false,
      render_login: false,
    };
  }
  
  handleHomeClick() {
    this.setState({
      render_home: true,
      render_details: false,
      render_contact: false,
      render_login: false
    });
  }
  
  handleDetailsClick() {
    this.setState({
      render_home: false,
      render_details: true,
      render_contact: false,
      render_login: false
    });
  }
  
  handleContactClick() {
    this.setState({
      render_home: false,
      render_details: false,
      render_contact: true,
      render_login: false
    });
  }
  
  handleLoginClick() {
    this.setState({
      render_home: false,
      render_details: false,
      render_contact: false,
      render_login: true
    });
  }
  
  renderHeader() {
    return (
      <div className="btn-group">
        <button onClick={this.handleHomeClick}>Home</button>
        <button onClick={this.handleDetailsClick}>Details</button>
        <button onClick={this.handleContactClick}>Contact Us</button>
        <button onClick={this.handleLoginClick}>Login</button>
      </div>
    )
  }
  
  renderHome() {
    return (
      <div>
        <h1>This is home</h1>
        <h3>Welcome home!</h3>
      </div>
    )
  }
  
  renderDetails() {
    return (
      <div>
        <h1>This is details</h1>
        <h3>Have some details</h3>
      </div>
    )
  }
  
  renderContacts() {
    return (
      <div>
        <h1>ContactUs</h1>
        <h3>Do it now!</h3>
      </div>
    )
  }
  
  render() {
    let page_data;
    if (this.state.render_home) {
      page_data = <this.renderHome />;
    } else if (this.state.render_details) {
      page_data = <this.renderDetails />;
    } else if (this.state.render_contact) {
      page_data = <this.renderContacts />;
    } else if (this.state.render_login) {
      page_data = <LoginManager />
    }
    
    let headerbuttons = <this.renderHeader/>
    
    return (
      <div>
        {headerbuttons}
        {page_data}
      </div>
    )
  }
}

export default PublicPages;
