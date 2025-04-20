import React from 'react';    
import { HashRouter as Router, Routes, Route } from 'react-router-dom'; 
import Login from './pages/login';
import Register from './pages/register';
import Dashboard from './pages/dashboard';
import Books from './pages/books';
import Collections from './pages/collections';

const App = () => {
  const handleLogin = (userData) => {
    console.log('User logged in:', userData);
  };

  return (
    <Router>
      <Routes>
        <Route path="/login" element={<Login onLogin={handleLogin} />} />
        <Route path="/register" element={<Register />} />
        <Route path="/dashboard" element={<Dashboard />} />
        <Route path="/books" element={<Books />} />
        <Route path="/collections" element={<Collections />} />

      </Routes>
    </Router>
  );
};

export default App;