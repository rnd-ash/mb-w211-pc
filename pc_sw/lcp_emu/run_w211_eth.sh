#!/bin/bash


USER="mercedes"
HOST=10.42.0.209
DEST="/home/mercedes/"
APP="lcp_emu"

#trap ctrl_c INT

#function ctrl_c() {
#    echo "Restarting"
#    ssh $USER@$HOST "systemctl --user restart $SERVICE"
#    echo "Done"
#}

cargo build --release
if [ $? -eq 0 ]; then
  ssh $USER@$HOST "pkill -f lcp_emu"
  rsync --progress target/release/$APP $USER@$HOST:$DEST
  ssh -t $USER@$HOST "DISPLAY=:0 RUST_LOG='debug' $DEST$APP;"
else
  echo "Compilation failed"
fi