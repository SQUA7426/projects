local socket = require 'socket'

local client_lib = {}
local nick_regex = "[%w]+%$"

function client_lib:valid_nickname()
  if type(self.nickname) ~= "string" then return false end

  if not self.nickname:match(nick_regex) then
    self.nickname = "nil"
    return false
  end
  return true
end

function client_lib:pack() return "<" .. self.nickname .. "><" .. self.ip .. "><" .. self.port .. ">" end

function client_lib:upack(msg_recv)
  local n, i, p = msg_recv:match("<(" .. nick_regex .. ")><([%d%.]+)><(%d+)>")
  if n and i and p then
    self.nickname, self.ip, self.port = n, i, tonumber(p)
    return { self.nickname, self.ip, self.port }
  end
  return { nil, nil, nil }
end

---@param dst_ip string
---@param port integer
---@param msg string
function client_lib:client_tcp(dst_ip, port, msg)
  print "Creating TCP Socket"
  local client = assert(socket.tcp())
  print("Connecting to " .. dst_ip .. " : " .. port)
  local success, err = client:connect(dst_ip, port)
  if not success then
    print('Connection Error: ' .. tostring(err))
    os.exit()
  end

  local sockname, sock_port = client:getsockname()
  if sockname and sock_port then self.ip, self.port = sockname, tonumber(sock_port) end
  local msg_fin = msg or client_lib:pack()

  print("Sending: " .. msg_fin)
  client:send(msg_fin .. "\n")

  local resp, err_rec, partial = client:receive()
  if not err_rec then
    print('Server responded: ' .. tostring(resp))

    print "SENDING CLOSE Cmd..."
    client:send("CLOSE")


    local close_recv, cl_err = client:receive()

    if not close_recv then print "CLOSING ACCEPTED..." end


  elseif err_rec == "closed" or err_rec == "timeout" then print(tostring(err_rec)) end
  client:close()
end

function client_lib:create_client(new_name, new_ip, new_port)
  local peer_client = setmetatable({
    nickname = new_name or "user", ip = new_ip or "127.0.0.1", port = new_port or 5000 }, { __index = client_lib })

  -- if new_name and not peer_client:valid_nickname() then
  --   print("INVALID Nickname: " .. peer_client.nickname)
  -- else
  --   print("VALID Nickname: " .. peer_client.nickname)
  -- end
  return peer_client
end

local client = client_lib:create_client()

return client
