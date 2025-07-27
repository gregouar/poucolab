/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')

module.exports = {
    content: {
      files: ["*.html", "./src/**/*.rs"],
      transform: {
        rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
      },
    },
    theme: {
      extend: {
        fontFamily: {
          sans: ["Inter", "system-ui", "sans-serif"],
        },
      },
    },
    plugins: [],
  }
