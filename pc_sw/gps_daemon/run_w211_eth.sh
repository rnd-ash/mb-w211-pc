#!/bin/bash


USER="mercedes"
HOST=10.42.0.209
#HOST="192.168.170.20"
SERVICE="daughter_pcb.service"
DEST="/mercedes/"
APP="w211_gps_service"

#trap ctrl_c INT

#function ctrl_c() {
#    echo "Restarting"
#    ssh $USER@$HOST "systemctl --user restart $SERVICE"
#    echo "Done"
#}

cargo build --release
if [ $? -eq 0 ]; then
  #ssh $USER@$HOST "systemctl --user stop $SERVICE"
  rsync --progress target/release/$APP launch_gps_service.sh  $USER@$HOST:$DEST
  ssh -t $USER@$HOST "RUST_LOG='debug' $DEST$APP;"
  echo "Killing"
  ssh $USER@$HOST "pkill -f 'w211_gps'"
  #echo "Restarting"
  #ssh $USER@$HOST "systemctl --user restart $SERVICE"
else
  echo "Compilation failed"
fi