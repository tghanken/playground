load("@aspect_rules_js//js:defs.bzl", "js_library")
load("@npm//projects/pocket-cellar/site:wrangler/package_json.bzl", wrangler_bin = "bin")

SRC_PATTERNS = [
    "**/*.css",
    "**/*.html",
    "**/*.svelte",
    "**/*.ts",
]

GENERATED_SRCS = [
    "lib/paraglide/**"
]

TEST_PATTERNS = [
    "*.test.ts",
    "*.spec.ts",
]

wrangler_bin.wrangler(
    name = "wrangler-types",
    srcs = ["//projects/pocket-cellar/site:wrangler"],
    args = ["types"],
    chdir = package_name(),
    outs = ["worker-configuration.d.ts"],
)

js_library(
    name = "src",
    srcs = glob(
        include = SRC_PATTERNS + [":wrangler-types"],
        allow_empty = True,
        exclude = GENERATED_SRCS + TEST_PATTERNS,
        ),
    visibility = ["//projects/pocket-cellar/site:__subpackages__"],
)
