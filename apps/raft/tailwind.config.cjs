/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.tsx', './index.html'],
    theme: {
        extend: {
            keyframes: {
                wiggle: {
                    '0%, 100%': { transform: 'rotate(-0.5deg)' },
                    '50%': { transform: 'rotate(0.5deg)' },
                },
            },
            animation: {
                wiggle: 'wiggle 200ms ease-in-out infinite',
            },
        },
    },
};
