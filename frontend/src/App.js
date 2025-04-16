import React from 'react';    
import { HashRouter as Router, Routes, Route } from 'react-router-dom'; //can use BrowerRouter later if needed
import {Login} from './pages/login';

const App=() => {
  return(
    <Router>
      <Routes>
        <Route path="/login" element={<Login />} />
      </Routes>
    </Router>
  )

};
    
export default App;
