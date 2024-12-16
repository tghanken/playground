load("@aspect_rules_js//js:defs.bzl", "js_library")

TRANSLATION_PATTERNS = [
    "*.json",
]

js_library(
    name = "messages",
    srcs = glob(TRANSLATION_PATTERNS),
    visibility = ["//projects/pocket-cellar/site:__subpackages__"],
)
