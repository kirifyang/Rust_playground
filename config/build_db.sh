#!/bin/bash
cd "${USERS_FILE_DIR}" || exit

# Download the opensky data
echo "Downloading data..."
wget -O- https://zenodo.org/records/5092942 | grep -oE 'https://zenodo.org/records/5092942/files/flightlist_[0-9]+_[0-9]+\.csv\.gz' | xargs wget
