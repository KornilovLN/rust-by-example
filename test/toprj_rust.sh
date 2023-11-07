#!/bin/bash

PTH=~/dev/rust
PRG="tutor"

# common info
echo; echo "<<< common_info >>>"
pwd
whoami

# go to rust 
echo; echo "<<< goto_rust >>>"
cd $PTH
ls -al

# is createted new prj_folder ???
if [ -e $PRG ]
then
echo; echo "<<< folder $PRG already exists >>>"
else
# create new prj_folder
echo; echo "<<< create_new_prj_folder >>>"
cargo new $PRG
fi

# goto prj_folder
echo; echo "<<< goto_prj_folder: $PRG >>>"
cd $PRG

# prj is created
echo; echo "<<< prj $PRG created >>>"
ls -al
