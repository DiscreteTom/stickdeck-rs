#!/bin/bash
konsole -e "/bin/bash -c '(env RUST_LOG=info,stickdeck=debug ./launch.sh || true) && read -p \"Press enter to exit\"'"
