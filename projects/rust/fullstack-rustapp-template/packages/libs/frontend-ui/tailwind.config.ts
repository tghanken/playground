import type {Config} from 'tailwindcss'
import daisyui from "daisyui";

const baseTheme = {
    "primary": "#864677",
    "secondary": "#b48ead",
    "accent": "#D5A021",
    "neutral": "#0B4F6C",

    "info": "#08B2E3",
    "success": "#6bb187",
    "warning": "#dbae59",
    "error": "#ac3e31",

    "--rounded-box": "0.5rem", // border radius rounded-box utility class, used in card and other large boxes
    "--rounded-btn": "0.25rem", // border radius rounded-btn utility class, used in buttons and similar element
    "--rounded-badge": "1rem", // border radius rounded-badge utility class, used in badges and similar
    "--animation-btn": "0.15s", // duration of animation when you click on button
    "--animation-input": "0.15s", // duration of animation for inputs like checkbox, toggle, radio, etc
    "--btn-focus-scale": "0.98", // scale transform of button when you focus on it
    "--border-btn": "1px", // border width of buttons
    "--tab-border": "1px", // border width of tabs
    "--tab-radius": "0.25rem", // border radius of tabs
}

const clight = {
    clight: {
        ...baseTheme,
        "base-100": "#eceff4",
        "base-200": "#CBD3E1",
        "base-300": "#97A8C3",
    }
}
const cdark = {
    cdark: {
        ...baseTheme,
        "base-100": "#202020",
        "base-200": "#141414",
        "base-300": "#0A0A0A",
    }
}

export default {
    mode: "all",
    content: [
        "./src_ts/**/*.ts",
        "./templates/**/*.html"
    ],
    theme: {
        extend: {},
    },
    plugins: [daisyui],
    daisyui: {
        themes: [clight, cdark],
        darkTheme: "cdark",
    },
    darkMode: ['selector', '[data-theme="cdark"]']
} satisfies Config