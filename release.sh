#!/bin/bash

# Build the image
docker buildx build -t dessalines/torrents-csv-leptos --platform linux/amd64,linux/arm64 . -f docker/Dockerfile

# Push it
docker push dessalines/torrents-csv-leptos
