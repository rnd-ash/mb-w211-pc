#!/bin/bash

function spin_up_can_iface() {
    operstate=$(cat /sys/class/net/${1}/operstate)
    if [ "$operstate" == 'down' ]
    then
        echo "CAN Iface is down! Trying to boot up"
        res=$(/usr/bin/ip link set $1 up)
        if [ $? -eq 0 ]
        then
            echo "CAN Iface $1 Setup completed OK!"
            return 0
        else
            echo "CAN Iface $1 could not be set to up"
            return 1
        fi
    else
        echo "CAN Iface is up already. No need to do anything."
        return 0
    fi
}

function setup_can_iface() {
    if [ ! -d "/sys/class/net/$1" ]
    then
        echo "CAN iface $1 does not exit. Spinning up"
        res=$(/usr/bin/ip link add dev $1 type vcan)
        if [ $? -eq 0 ]
        then
            spin_up_can_iface "$1"
            return $?
        else
            echo "CAN Iface $1 could not be initialized"
            return 1
        fi
    else
        echo "CAN iface $1 exists!"
        spin_up_can_iface "$1"
        return $?
    fi
}

setup_can_iface "vcan_b"
setup_can_iface "vcan_c"
setup_can_iface "vcan_e"

/mercedes/w211_can_service