import * as Sentry from "@sentry/browser";

Sentry.init({
    dsn: "https://290896d656fd90187b86d6fec7d3dc28@o4507971674570752.ingest.us.sentry.io/4507971768680448",
    maxBreadcrumbs: 50,
    debug: false,
    release: process.env.npm_package_version,
    environment: process.env.SENTRY_ENV,
    tracesSampleRate: 1.0,
    integrations: [Sentry.browserTracingIntegration()],
    tracePropagationTargets: ["localhost", /^https:\/\/fullstack-rustapp-template.fly.dev/],
});