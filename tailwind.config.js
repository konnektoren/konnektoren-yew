/** @type {import('tailwindcss').Config} */

import daisyui from "daisyui";

module.exports = {
  content: ["./src/**/*.{html,js,rs}", "./index.html"],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
  daisyui: {
    themes: ["light", "dark"],
  },
};
