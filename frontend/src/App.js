import React from 'react';    
import { HashRouter as Router, Routes, Route } from 'react-router-dom'; 
import { Login } from './pages/login';
import { Register } from './pages/register';

const App = () => {
  const handleLogin = (userData) => {
    console.log('User logged in:', userData);
  };

  return (
    <Router>
      <Routes>
        <Route path="/login" element={<Login onLogin={handleLogin} />} />
        <Route path="/register" element={<Register />} />
      </Routes>
    </Router>
  );
};

export default App;