import react, {useState} from 'react';
//import '.../css/login.css;

const GoogleFontsStyle = `
    @import url('https://fonts.googleapis.com/css2?family=Space+Mono:wght@700&display=swap');
`;

export const Login = () => {
    const [formData, setFormData] = useState({
        username: '',
        password: '',
    })

    const [message, setMesage] = useState('');

    const handleInputChange = (e) => {
        const {name, value} = e.target;
        setFormData((prev) => ({
            ...prev,
            [name]: value,
        })) 
    };

    const HnadleFormSubmmit= async(event) => {
        event.preventDefualt();

        setMessage('Signing in!');

        setTimeout(() => {
            if(formData.username === 'admin'&& formData.password === 'pass123'){
                setMessage('Login successful!');
            } else {
                setMesage('Invalid Login');  
            }
        }, 1000); // 1 second delay

    };

    return(
        <div className ='wrap'>
            <div className= {'login-box space-mono-bold'}>
                <h3>Login</h3>
                <form onSubmit= {HnadleFormSubmmit}>
                    <div></div>
                </form>
            </div>
        </div>
    )


};








