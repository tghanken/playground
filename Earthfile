VERSION --build-auto-skip 0.8
PROJECT tghanken/playground

# Core Targets - Intended to be called by a user
install:
    BUILD ./projects+install

build:
    BUILD --auto-skip ./projects+build

check:
    BUILD --auto-skip +build
    BUILD --auto-skip ./projects+check

fix:
    BUILD ./projects+fix

deploy:
    BUILD --auto-skip +check
    BUILD --auto-skip ./projects+deploy

deploy-all:
    BUILD --auto-skip +check
    BUILD ./projects+deploy-all

# Utility Targets - Intended to be called only by core targets
