#!/usr/bin/make

ARGS =
DIR = ./
SRC = ./src/examples/

CLIENT = client
SERVER = server
IP =

.PHONY:
	make sync

sync:
	cp -R $(SRC) $(DIR)

client:
	cargo run --example $(CLIENT) $(ARGS)

server:
	cargo run --example $(SERVER) $(IP)
