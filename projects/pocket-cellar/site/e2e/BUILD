load("@aspect_rules_js//js:defs.bzl", "js_library")

TEST_PATTERNS = [
    "*.test.ts",
    "*.spec.ts",
]

js_library(
    name = "e2e_test_files",
    srcs = glob(TEST_PATTERNS),
    visibility = ["//projects/pocket-cellar/site:__subpackages__"],
)
