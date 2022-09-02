/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./tools/**/*.{html,js,rs}",
    "./server/static/**/*.{html,js.rs}",
    "./test.html"
  ],
  theme: {
    extend: {
      keyframes:{
        fallin: {
          '0%': {transform:'translateY(calc(-10vh))', opacity:'0'},
          '100%': {transform:'translateY(0)', opacity:'1'}
        },
        fadin:{
          '0%': {opacity:'0'},
          '100%': {opacity:'1'}
        },
        invis: {
          '0%': {opacity: '0'},
          '100%': {opacity:'0'}
        }
      },
      animation:{
        fallin: 'fallin 0.7s ease-in-out 1'
      },
      colors:{
        navy:{
          900: '#061D33',
          800: '#082244',
          700: '#0B345B',
          600: '#0F4272',
          500: '#215382',
          400: '#325E92',
          300: '#5F85B3',
          200: '#7A9CC4',
          100: '#9AB6D6',
          50: '#D3E1F2',
        }
      }
    },
  },
  plugins: [],
}
