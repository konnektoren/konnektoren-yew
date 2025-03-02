/** @type {import('tailwindcss').Config} */

import daisyui from "daisyui";

module.exports = {
  content: ["./src/**/*.{html,js,rs}", "./index.html"],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
  daisyui: {
    themes: [
      {
        light: {
          primary: "#ff8c00",
          "primary-content": "#ffffff",
          secondary: "#8a2be2",
          "secondary-content": "#ffffff",
          accent: "#808080",
          "accent-content": "#ffffff",
          neutral: "#808080",
          "neutral-content": "#ffffff",
          "base-100": "#f5f5f5",
          "base-200": "#e9ecef",
          "base-300": "#dee2e6",
          "base-content": "#262626",
          info: "#17a2b8",
          "info-content": "#ffffff",
          success: "#28a745",
          "success-content": "#ffffff",
          warning: "#ffc107",
          "warning-content": "#262626",
          error: "#dc3545",
          "error-content": "#ffffff",
        },
      },
      {
        dark: {
          primary: "#ffa500",
          "primary-content": "#ffffff",
          secondary: "#9932cc",
          "secondary-content": "#ffffff",
          accent: "#a0a0a0",
          "accent-content": "#ffffff",
          neutral: "#808080",
          "neutral-content": "#ffffff",
          "base-100": "#121212",
          "base-200": "#343a40",
          "base-300": "#495057",
          "base-content": "#f5f5f5",
          info: "#03dac6",
          "info-content": "#ffffff",
          success: "#4caf50",
          "success-content": "#ffffff",
          warning: "#ffb74d",
          "warning-content": "#262626",
          error: "#ff5252",
          "error-content": "#ffffff",
        },
      },
      "cyberpunk",
    ],
  },
};
