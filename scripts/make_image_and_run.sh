#!/bin/bash

scripts/make_image.sh $1
qemu-system-x86_64 -cdrom target/os.iso