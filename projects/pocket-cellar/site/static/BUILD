load("@aspect_rules_js//js:defs.bzl", "js_library")

SRC_PATTERNS = [
    "**/*.png",
]

js_library(
    name = "static",
    srcs = glob(
        include = SRC_PATTERNS,
        ),
    visibility = ["//projects/pocket-cellar/site:__subpackages__"],
)
