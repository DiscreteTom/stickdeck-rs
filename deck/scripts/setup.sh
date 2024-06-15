#!/bin/bash
konsole -e "sudo cp ./libsteam_api.so /usr/lib && mkdir -p /home/deck/.local/share/Steam/controller_config && cp ./stickdeck.vdf /home/deck/.local/share/Steam/controller_config/game_actions_480.vdf"
