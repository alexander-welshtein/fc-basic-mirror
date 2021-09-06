const Color = require('color')

const lighten = (color, value) => Color(color).lighten(value).rgb().string()
const darken = (color, value) => Color(color).darken(value).rgb().string()

module.exports = {
  purge: [],
  darkMode: false,
  theme: {
    extend: {
      colors: {
        accent: {
          default: '#16A085',
          lighten: lighten('#16A085', .1),
          'darken-a': darken('#16A085', .1),
          'darken-b': darken('#16A085', .2)
        }
      }
    }
  },
  variants: {
    extend: {
      backgroundColor: ['active']
    },
  },
  plugins: [],
}
