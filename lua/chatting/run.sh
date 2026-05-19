#!/usr/bin/sh
lua src/server.lua &
sleep 3
kitty --hold sh -c 'lua main.lua'
exit
