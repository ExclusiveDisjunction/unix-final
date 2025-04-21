import React, {useState} from 'react';
import { Link, useNavigate } from 'react-router-dom'; 
import '../css/login.css';

const GoogleFontsStyle = `
    @import url('https://fonts.googleapis.com/css2?family=Space+Mono:wght@700&display=swap');
`;

function Login({ onLogin }) {
    const navigate = useNavigate();
    const [message, setMessage] = useState('');
    const [error, setError] = useState('');
    const [formData, setFormData] = useState({
        username: '',
        password: '',
    });
    const handleInputChange = (e)=> {
        setFormData({...formData, [e.target.name]: e.target.value});
    };

    const handleFormSubmit= async(e)=>{
        e.preventDefault();
        setMessage('Logging in');

        try {
            const response= await fetch(`${process.env.REACT_APP_API_URL}/sign-in`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formData),
            });
            if (response.status=== 501){
                setError('Login not implemented');
                return;
            }
            if (!response.ok){
                setError('Invalid uername or password');
                return;
            }

            const token = await response.text();

            localStorage.setItem('token',token);
            navigate('/dahsboard');
        }catch (error) {
            setMessage('Error connecting to the server');
    }
    };

    return(
        <div className ='wrap'>
        <style>{GoogleFontsStyle}</style>
            <div className= {'login-form-box space-mono-bold'}>
                <h3>Login</h3>
                <form onSubmit= {handleFormSubmit}>
                    <div className= "input-box">
                        <span className="icon"><ion-icon name="perosn"></ion-icon></span>
                        <input 
                            type='text'
                            name='username'
                            value={formData.username} 
                            onChange={handleInputChange} 
                            required
                        />
                        <label>Username</label>
                    </div>
                    <div className= "input-box">
                        <span className="icon"><ion-icon name="lock-closed"></ion-icon></span>
                        <input 
                            type='password' 
                            name='password' 
                            value={formData.password} 
                            onChange={handleInputChange} 
                            required
                        />
                        <label>Password</label>
                    </div>
                    <button type='submit' className='bttn space-mono-bold'>Login</button>
                    <div className= 'account-register'>
                        <a href='#register'>Dont have an account? Register</a>
                    </div>
                    <div id="info"></div>
                </form>
            </div>
        </div>
    )


};

export default Login;








