load("@aspect_rules_js//js:defs.bzl", "js_library")

TRANSLATION_PATTERNS = [
    "*.png",
]

js_library(
    name = "static",
    srcs = glob(TRANSLATION_PATTERNS),
    visibility = ["//projects/pocket-cellar/site:__subpackages__"],
)
