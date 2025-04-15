import react, {useState} from 'react';
//import '.../css/login.css;

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


};





