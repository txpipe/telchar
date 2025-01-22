import type { Config } from 'tailwindcss';

export default {
  content: ['./app/**/{**,.client,.server}/**/*.{js,jsx,ts,tsx}'],
  darkMode: 'selector',
  theme: {
    extend: {
      fontFamily: {
        sans: [
          '"Inter"',
          'ui-sans-serif',
          'system-ui',
          'sans-serif',
          '"Apple Color Emoji"',
          '"Segoe UI Emoji"',
          '"Segoe UI Symbol"',
          '"Noto Color Emoji"',
        ],
        roboto: [
          '"Roboto Mono"', 'serif',
        ],
      },
      colors: {
        primary: {
          // TBD
          400: '#F41F95',
          500: '#B10151',
        },
        secondary: {
          // TBD
          400: '#3BE0FE',
          500: '#0094AE',
        },
      },
      margin: {
        22: '5.5rem',
      },
      padding: {
        4.5: '1.125rem',
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
} satisfies Config;
