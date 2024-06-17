#!/bin/bash
konsole -e "/bin/bash -c 'export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:. && (./stickdeck || true) && read -p \"Press enter to exit\"'"
