load("@aspect_rules_js//js:defs.bzl", "js_library")

SRC_PATTERNS = [
    "**/*.css",
    "**/*.html",
    "**/*.svelte",
    "**/*.ts",
    "**/*.d.ts",
]

GENERATED_SRCS = [
    "lib/paraglide/**"
]

TEST_PATTERNS = [
    "*.test.ts",
    "*.spec.ts",
]

js_library(
    name = "src",
    srcs = glob(
        include = SRC_PATTERNS,
        allow_empty = True,
        exclude = GENERATED_SRCS + TEST_PATTERNS,
        ),
    visibility = ["//projects/pocket-cellar/site:__subpackages__"],
)
