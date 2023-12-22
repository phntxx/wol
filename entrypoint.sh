#!/bin/sh
set -eu

# This script checks if the default needed files are
# available at the specified location (defaults specified via env-vars)
# If they're not, it copies the default files there.

if [[ ! -f $CONFIG_FILE ]]; then
  __DIRNAME="$(dirname $CONFIG_FILE)"
  mkdir -p $__DIRNAME
  cp defaults/config.yml $CONFIG_FILE || true
else
  rm -rf defaults/config.yml
fi

if [[ ! "$(ls -a defaults)" ]]; then
  rm -rf defaults
fi

"$@"