import React, { useState } from 'react';

const BookForm = () => {

    const [isFormVisible, setIsFormVisible] = useState(false)

    const [formData, setFormData] = useState({
    title: '',
    author: '',
    groupName: '',
    rating: 1, // Default rating
    isFavorite: false,
    genre: '',
    });

    const handleChange = (e) => {
        const { name, value, type, checked } = e.target;
        setFormData((prevData) => ({
            ...prevData,
            [name]: type === 'checkbox' ? checked : value,
        }));
    };

    const handleSubmit = async (e) => {
        e.preventDefault();
    

    const bookData = {
        title: formData.title,
        author: formData.author,
        groupName: formData.groupName,
        rating: parseInt(formData.rating), // Ensure rating is a number
        isFavorite: formData.isFavorite,
        genre: formData.genre,
    };

    try {
    const response = await fetch(`${process.env.REACT_APP_API_URL}/{username}/add-book`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(bookData),
    });

    if (response.ok) {
        console.log('Book added successfully:', bookData);
        setFormData({
            title: '',
            author: '',
            groupName: '',
            rating: 1,
            isFavorite: false,
            genre: '',
        });
        // Hide the form after submission
        setIsFormVisible(false);
    } else {
        console.error('Failed to add book:', response.statusText);
    }
    } catch (error) {
        console.error('Error submitting book:', error);
        console.log('Book data (fallback):', bookData);

        const existingBooks = JSON.parse(localStorage.getItem('books') || '[]');
        localStorage.setItem('books', JSON.stringify([...existingBooks, bookData]));
        }
    };

    return (
        <div className="p-4">
        {/* Add Book Button */}
        {!isFormVisible && (
            <button
            onClick={() => setIsFormVisible(true)}
            className="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-600 transition-colors"
            >
            Add Book
            </button>
        )}

        {isFormVisible && (
            <form onSubmit={handleSubmit} className="max-w-md mx-auto bg-white p-6 rounded-lg shadow-md space-y-4">
            <div>
                <label htmlFor="title" className="block text-sm font-medium text-gray-700">
                Title
                </label>
                <input
                type="text"
                id="title"
                name="title"
                value={formData.title}
                onChange={handleChange}
                required
                className="mt-1 block w-full p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
            </div>
            <div>
                <label htmlFor="author" className="block text-sm font-medium text-gray-700">
                Author
                </label>
                <input
                type="text"
                id="author"
                name="author"
                value={formData.author}
                onChange={handleChange}
                required
                className="mt-1 block w-full p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
            </div>

            <div>
                <label htmlFor="groupName" className="block text-sm font-medium text-gray-700">
                Group Name
                </label>
                <input
                type="text"
                id="groupName"
                name="groupName"
                value={formData.groupName}
                onChange={handleChange}
                required
                className="mt-1 block w-full p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
            </div>
            <div>
                <label htmlFor="rating" className="block text-sm font-medium text-gray-700">
                Rating (1-5)
                </label>
                <input
                type="number"
                id="rating"
                name="rating"
                min="1"
                max="5"
                value={formData.rating}
                onChange={handleChange}
                required
                className="mt-1 block w-full p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
            </div>
            <div className="flex items-center">
                <input
                type="checkbox"
                id="isFavorite"
                name="isFavorite"
                checked={formData.isFavorite}
                onChange={handleChange}
                className="h-4 w-4 text-blue-500 focus:ring-blue-500 border-gray-300 rounded"
                />
                <label htmlFor="isFavorite" className="ml-2 text-sm font-medium text-gray-700">
                Favorite
                </label>
            </div>
            <div>
                <label htmlFor="genre" className="block text-sm font-medium text-gray-700">
                Genre
                </label>
                <input
                type="text"
                id="genre"
                name="genre"
                value={formData.genre}
                onChange={handleChange}
                required
                className="mt-1 block w-full p-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
            </div>
            <div className="flex space-x-4">
                <button
                type="submit"
                className="bg-green-500 text-white px-4 py-2 rounded hover:bg-green-600 transition-colors"
                >
                Submit
                </button>
                <button
                type="button"
                onClick={() => setIsFormVisible(false)}
                className="bg-gray-500 text-white px-4 py-2 rounded hover:bg-gray-600 transition-colors"
                >
                Cancel
                </button>
            </div>
            </form>
        )}
        </div>
    );
};

export default BookForm;