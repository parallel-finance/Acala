#!/usr/bin/env bash

set -e

VERSION=v0.9.16
NODE_NAME=parallelfinance/karura
BUILD_ARGS="--features with-karura-runtime"

docker build -f scripts/Dockerfile . -t $NODE_NAME:$VERSION --build-arg GIT_COMMIT=${VERSION} --build-arg BUILD_ARGS="$BUILD_ARGS"
docker push $NODE_NAME:$VERSION
