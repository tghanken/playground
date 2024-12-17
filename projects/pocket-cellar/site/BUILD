# TODO: add unit tests instead of only testing building
load("@bazel_skylib//rules:build_test.bzl", "build_test")
load("@aspect_rules_js//js:defs.bzl", "js_library", "js_run_binary", "js_run_devserver", "js_test")
load("@npm//:defs.bzl", "npm_link_all_packages")
load("@npm//projects/pocket-cellar/site:vite/package_json.bzl", vite_bin = "bin")
# load("@npm//projects/pocket-cellar/site:svelte-kit/package_json.bzl", svelte_kit_bin = "bin")

# This macro expands to a link_npm_package for each third-party package in package.json
npm_link_all_packages(name = "node_modules")

SRCS = [
    "package.json",
    "svelte.config.js",
    "tailwind.config.ts",
    "tsconfig.json",
    "vite.config.ts",
    "wrangler.toml",
    "//projects/pocket-cellar/site/src",
]

# HOISTED_DEPS = ["//projects:node_modules/" + d for d in [
#     "@sinclair/typebox",
# ]]

BUILD_DEPS = [":node_modules/" + d for d in [
    "@inlang/paraglide-sveltekit",
    "@sveltejs/kit",
    "tailwindcss",
    "vite",
    "vitest",
    # "@bazel-example/vue-library",
    # "@vitejs/plugin-vue",
    # "@vitejs/plugin-vue-jsx",
    # "@vue/tsconfig",
    # "vite",
    # "vue",
    # "vue-router",
]]

vite_bin.vite(
    name = "build",
    srcs = SRCS + BUILD_DEPS,
    args = ["build"],
    chdir = package_name(),
    out_dirs = ["dist"],
)

# vue_tsc_bin.vue_tsc_test(
#     name = "type-check",
#     args = ["--noEmit"],
#     chdir = package_name(),
#     data = SRCS + BUILD_DEPS,
#     include_types = True,
# )

# vite_bin.vite_binary(
#     name = "vite",
#     chdir = package_name(),
#     data = SRCS + BUILD_DEPS,
#     # Under ibazel, let vite hot-reload changed files rather than restart it
#     tags = ["ibazel_notify_changes"],
# )

build_test(
    name = "build_test",
    targets = [
        ":build",
        # ":type-check",
    ],
)