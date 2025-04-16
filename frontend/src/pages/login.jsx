import react, {useState} from 'react';
import '../css/login.css';

const GoogleFontsStyle = `
    @import url('https://fonts.googleapis.com/css2?family=Space+Mono:wght@700&display=swap');
`;

export const Login = () => {
    const [formData, setFormData] = useState({
        username: '',
        password: '',
    })

    const [message, setMessage] = useState('');

    const handleInputChange = (e) => {
        const {name, value} = e.target;
        setFormData((prev) => ({
            ...prev,
            [name]: value,
        })) 
    };

    const handleFormSubmmit= async(event) => {
        event.preventDefault();

        setMessage('Signing in!');

        setTimeout(() => {
            if(formData.username === 'admin'&& formData.password === 'pass123'){
                setMessage('Login successful!');
            } else {
                setMessage('Invalid Login');  
            }
        }, 1000); // 1 second delay

    };

    return(
        <div className ='wrap'>
            <div className= {'login-form-box space-mono-bold'}>
                <h3>Login</h3>
                <form onSubmit= {handleFormSubmmit}>
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
                        <a href='#'>Dont have an account? Register</a>
                    </div>
                    <div id="info"></div>
                </form>
            </div>
        </div>
    )


};

export default Login;








