#!/bin/bash

case $1 in
  "basicapp")
    echo "Executing BasicApp demo."
    ;;
  *)
    echo "Usage: $0 [basicapp]"
    ;;
esac