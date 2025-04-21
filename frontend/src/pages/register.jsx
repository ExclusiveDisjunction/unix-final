import React, { useState } from 'react';
import '../css/register.css';
import { useNavigate } from 'react-router-dom';

const GoogleFontsStyle = `
    @import url('https://fonts.googleapis.com/css2?family=Space+Mono:wght@700&display=swap');
`;

export const Register = () => {
    const navigate = useNavigate
    const [message, setMessage] = useState('');
    
    const [formData, setFormData] = useState({
        username: '',
        password: '',
        confirmPassword: '',
    });
    const [error, setError] = useState('');

    const handleInputChange = (e) => {
        setFormData((prev) => ({
            ...prev,
            [e.target.name]: e.target.value,
        }));
    };

    const handleFormSubmit = async (e) => {
        e.preventDefault();
        setError('');

        try{
            const response = await fetch(`${process.env.REACT_APP_API_URL}/create-user`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formData),
            });

            if (response.status === 409) {
                setError('User already exist');
                return;
            }
            if (!response.ok) {
                setError('Failed to Register');
                return;
            }

            const token = await response.text();
            localStorage.setItem('token', token);
            navigate('/login');
        } catch (error) {
            setError('Error connecting to the server');
        }
    };

    return (
        <div className="wrap-register">
            <style>{GoogleFontsStyle}</style>
            <div className="register-form-box space-mono-bold">
                <h3>Register</h3>
                <form onSubmit={handleFormSubmit}>
                    <div className="input-box-register">
                        <span className="icon"><ion-icon name="person"></ion-icon></span>
                        <input
                            type="text"
                            name="username"
                            value={formData.username}
                            onChange={handleInputChange}
                            required
                        />
                        <label>Username</label>
                    </div>
                    <div className="input-box-register">
                        <span className="icon"><ion-icon name="lock-closed"></ion-icon></span>
                        <input
                            type="password"
                            name="password"
                            value={formData.password}
                            onChange={handleInputChange}
                            required
                        />
                        <label>Password</label>
                    </div>
                    <div className="input-box-register">
                        <span className="icon"><ion-icon name="lock-closed"></ion-icon></span>
                        <input
                            type="password"
                            name="confirmPassword"
                            value={formData.confirmPassword}
                            onChange={handleInputChange}
                            required
                        />
                        <label>Confirm Password</label>
                    </div>
                    <button type="submit" className="bttn-register space-mono-bold">Register</button>
                    <div className="account-login">
                        <p>Already have an account?</p>
                        <a href="#login">Login</a>
                    </div>
                    <div id="info">{message}</div>
                </form>
            </div>
        </div>
    );
};

export default Register;