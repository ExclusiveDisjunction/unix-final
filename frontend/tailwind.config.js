/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{js,jsx,ts,tsx}"],
  theme: {
    extend: {
      colors: {
        'custom-green': {
          400: '#22aa64',
          500: '#38d668',
        },
      },
    },
  },
  plugins: [],
};

