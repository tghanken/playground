import type {Config} from 'tailwindcss'
import daisyui from "daisyui";

export default {
    mode: "all",
    content: [
        "./src/**/*.{rs,html,css,js,ts}",
        "./dist/**/*.html",
        "./templates/**/*.html"
    ],
    theme: {
        extend: {},
    },
    plugins: [daisyui],
    daisyui: {},
} satisfies Config