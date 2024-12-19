# Make sure to call BUILD --auto-skip only on leaf targets doing actual work, not aggregation targets.
# Ex. BUILD --auto-skip ./projects+build is not ok, but BUILD --auto-skip ./projects/pocket-cellar/site+build is ok.
VERSION --build-auto-skip 0.8
PROJECT tghanken/playground

build:
    BUILD ./projects+build

check:
    BUILD ./projects+check

deploy:
    BUILD ./projects+deploy
