import { defineConfig } from 'vite';
import { sentryVitePlugin } from "@sentry/vite-plugin";

// https://vitejs.dev/config/
export default defineConfig({
    html: {
      cspNonce: 'csp-placeholder'
    },
    build: {
        manifest: true,
        sourcemap: "hidden",
        rollupOptions: {
            input: ['./src/main.ts', './src/sentry.ts'],
        }
    },
    plugins: [
        // Put the Sentry vite plugin after all other plugins
        sentryVitePlugin({
            disable: true,
            telemetry: false,
            org: process.env.SENTRY_ORG,
            project: process.env.SENTRY_PROJECT,
            release: {
                name: process.env.SENTRY_RELEASE_SHA,
                create: false,
                finalize: false,
                setCommits: {
                    auto: false,
                    commit: process.env.SENTRY_RELEASE_SHA,
                    repo: process.env.SENTRY_REPO,
                },
                deploy: {
                    env: process.env.SENTRY_ENV,
                }
            },

            // Auth tokens can be obtained from https://sentry.io/orgredirect/organizations/:orgslug/settings/auth-tokens/
            authToken: process.env.SENTRY_AUTH_TOKEN,
            sourcemaps: {
                // Can't upload within nix
                disable: true,
            }
        }),
    ]
})
