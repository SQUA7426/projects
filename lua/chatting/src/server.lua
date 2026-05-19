local socket = require 'socket'
-- local peer_client = require 'src.client'
local peer_client = require 'test'

local server_lib = {}

local peers = {}

function server_lib:chat_peer(b)
  local new_peers = {}
  for _, v in pairs(peers) do
    print("VALUE: " .. v)
    v = tostring(v)
    local n, t, p
    if not b then
      n, t, p = peer_client.USER_LIST_BEGIN(v, false)
      p = tonumber(p)
      print(n, t, p)
    else
      local r
      r, n, t, p = peer_client.USER_LIST_BEGIN(v, true)
      p = tonumber(p)
      print(r, n, t, p)
      table.insert(new_peers, peer_client.REGISTER(n, t, p))
    end
  end
  peers = new_peers
  for _, v in pairs(peers) do
    for k, i in pairs(v) do print(k, i) end
  end
end

function server_lib:closing(conn)
  local closing, close_err = conn:receive()

  if not close_err and closing == "CLOSE" then
    print("Received CLOSE CMD: " .. closing)
  end
end

function server_lib:recv(conn)
  local line, err_recv = conn:receive()

  if not err_recv then
    print("Received: " .. tostring(line))
    local start, ending = string.find(line, "nil")
    if start and ending then
      print "NIL FOUND"
    else
      table.insert(peers, line)

      conn:send("Echo: " .. line .. "\n")
    end
    server_lib:closing(conn)
    return false
  elseif err_recv == "closed" or err_recv == "timeout" then
    print("CONN - Err: " .. err_recv)
    return false
  end
  return true
end

function server_lib:server_tcp(server_ip, port)
  server_ip, port = server_ip or "127.0.0.1", port or 5000
  print "Create TCP Server"
  local server = assert(socket.tcp())
  print("Connecting TCP on IP:", server_ip, "Port:", port)
  server:bind(server_ip, port)
  print "Listens..."
  server:settimeout(5)
  server:listen(3)

  local running = true

  while running do
    local conn, err = server:accept()

    if conn then
      conn:settimeout(5)
      local ip, cport = conn:getpeername()
      print("Connected with socket: " .. ip .. ": " .. cport)

      running = server_lib:recv(conn)
    elseif err == "closed" or err == "timout" then
      print("SERVER - Err: " .. err)
      conn.close()
    end

    if conn then conn:close() end
    if not running then server:close() end
  end
end

function server_lib:new()
  local peer_server = setmetatable({}, { __index = server_lib })
  return peer_server
end

local server = server_lib:new()
server:server_tcp()
server:chat_peer(true)
-- server:server_tcp("255.0.0-- return server
