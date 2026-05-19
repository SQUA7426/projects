-- local peer_client = require 'src.client'
--
-- local user = peer_client:create_client("a2$")
-- local s = user:pack()
-- user:client_tcp(user.ip, user.port, s)
--
-- for k, v in pairs(subject) do print(k .. ": " .. tostring(v)) end
--
-- local n, t, p = subject:upack("<User123$><127.0.0.1><5000>", false)
-- print(n, t, p)


local cmds = require 'test'
local subject = cmds.REGISTER()
local ss = subject:pack("REGISTER",true)
-- print("PACKET: " .. ss)

subject:request(subject.ip, subject.port, ss)
local r, n, t, p = subject:upack(ss, true)
print(r, n, t, p)
