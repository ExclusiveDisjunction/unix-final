import React from 'react';    
import { HashRouter as Router, Routes, Route } from 'react-router-dom'; //can use BrowerRouter later if needed
import {Login} from './pages/login';
import {Register} from './pages/register';

const App=() => {
  return(
    <Router>
      <Routes>
        <Route path="/login" element={<Login onLogin={handleLogin}/>} />
        <Route path="/register" element={<Register />} />
      </Routes>
    </Router>
  )

};
    
export default App;
