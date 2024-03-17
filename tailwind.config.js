/** @type {import('tailwindcss').Config} */

export default {
  content: [
    './src/*.rs',
    './src/components/*.rs',
    './src/pages/*.rs',
  ],

  theme: {
    colors: {
      transparent: 'transparent',
      current: 'currentColor',

      gray: {
        400: '#928374',
        500: '#7c6f64',
        600: '#665c54',
        700: '#504945',
        800: '#3c3836',
        900: '#282828',
        950: '#1d2021',
        DEFAULT: '#282828',
      },

      white: {
        600: '#a89984',
        700: '#bdae93',
        800: '#d5c4a1',
        900: '#ebdbb2',
        950: '#fbf1c7',
        DEFAULT: '#ebdbb2',
      },

      red: {
        900: '#fb4934',
        950: '#cc2412',
      },

      orange: {
        900: '#fe8019',
        950: '#d65d0e',
      },

      yellow: {
        900: '#fabd2f',
        950: '#d79921',
      },

      green: {
        900: '#b8bb26',
        950: '#98971a',
      },

      blue: {
        900: '#83a598',
        950: '#458588',
      },

      purple: {
        900: '#d3869b',
        950: '#b16286',
      },

      aqua: {
        900: '#8ec07c',
        950: '#689d6a',
      },
    },
  },
  plugins: [],
}
