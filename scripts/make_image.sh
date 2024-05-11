#!/bin/bash

WORKDIR=`mktemp -d`

mkdir $WORKDIR/boot/grub -p
cp $1 $WORKDIR/boot/kernel.bin
cp grub_config/grub.cfg $WORKDIR/boot/grub/grub.cfg

grub-mkrescue -o target/os.iso $WORKDIR/
rm -r $WORKDIR