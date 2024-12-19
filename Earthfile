VERSION --build-auto-skip 0.8
PROJECT tghanken/playground

# Core Targets - Intended to be called by a user
build:
    BUILD --auto-skip ./projects+build

check:
    BUILD --auto-skip +build
    BUILD --auto-skip ./projects+check

fix:
    BUILD --auto-skip ./projects+fix

deploy:
    BUILD --auto-skip +check
    BUILD --auto-skip ./projects+deploy

# Utility Targets - Intended to be called only by core targets
