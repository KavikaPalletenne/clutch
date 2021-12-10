module.exports = {
  purge: [
  './pages/**/*.{js,ts,jsx,tsx}',
  './components/**/*.{js,ts,jsx,tsx}'
],
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      colors: {
        exclpurple: {
          DEFAULT: '#C290FF',
          dark: '#8C9AFF',
        },
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
