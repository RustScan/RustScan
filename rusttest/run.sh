#!/bin/bash
docker build -t rustscan-builder . || exit

# This creates a 'volume' which binds your currentdirectory/debs to 
# the location where the deb files get spat out in the container.
# You don't need to worry about it. Just chmod +x run.sh && ./run.sh and
# you'll get yer .deb file in a few minutes.
docker run -v "$(pwd)/debs:/RustScan/target/debian" rustscan-builder
