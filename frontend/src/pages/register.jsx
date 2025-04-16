import React, { useState } from 'react';
import '../css/register.css';

const GoogleFontsStyle = `
    @import url('https://fonts.googleapis.com/css2?family=Space+Mono:wght@700&display=swap');
`;

export const Register = () => {
    const [formData, setFormData] = useState({
        username: '',
        password: '',
        confirmPassword: '',
    });

    const [message, setMessage] = useState('');

    const handleInputChange = (e) => {
        const { name, value } = e.target;
        setFormData((prev) => ({
            ...prev,
            [name]: value,
        }));
    };

    const handleFormSubmit = async (event) => {
        event.preventDefault();

        setMessage('Registering...');

        setTimeout(() => {
            // Mock validation
            if (formData.username === '' || formData.email === '' || formData.password === '' || formData.confirmPassword === '') {
                setMessage('All fields are required.');
            } else if (formData.password !== formData.confirmPassword) {
                setMessage('Passwords do not match.');
            } else {
                // Simulate successful registration
                setMessage('Registration successful! You can now log in.');
            }
        }, 1000); // 1 second delay to mimic async request
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
                        <a href="/login">Login</a>
                    </div>
                    <div id="info">{message}</div>
                </form>
            </div>
        </div>
    );
};

export default Register;