#!/bin/bash
WINIT_X11_SCALE_FACTOR=1 LD_LIBRARY_PATH=$LD_LIBRARY_PATH:. ./stickdeck |& tee log.txt