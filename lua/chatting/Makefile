#!/usr/bin/ Makefile

.PHONY:
	make requirements dependencies

requirements:
	sudo pacman -S love && \
	sudo luarocks install luasocket && sudo luarocks install love2d.nvim

dependencies:
	echo -e '{\n\t"$$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",\n\t"runtime": {\n\t\t"version": "Lua 5.5"\n\t}\n}' > .luarc.json
