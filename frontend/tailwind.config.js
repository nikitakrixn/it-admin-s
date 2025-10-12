/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './app/components/**/*.{js,vue,ts}',
    './app/layouts/**/*.vue',
    './app/pages/**/*.vue',
    './app/plugins/**/*.{js,ts}',
    './app/app.vue',
  ],
  theme: {
    extend: {
      colors: {
        primary: {
          50: '#e3f2fd',
          100: '#90caf9',
          200: '#64b5f6',
          300: '#42a5f5',
          400: '#2196f3',
          500: '#1976d2',
          600: '#1565c0',
          700: '#0d47a1',
          800: '#003a7f',
          900: '#002f6c',
          950: '#002654',
        },
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}
