local socket = require 'socket'

local cmds = {
  REGISTER = function() end,
  REGISTER_OK = function() end,
  REGISTER_ERR = function() end,
  USER_LIST_BEGIN = function() end,
  USER_ITEM_ = function() end,
  USER_LIST_END = function() end,
  USER_LEFT = function() end,
  USER_JOIN = function() end,
  BROADCAST_SEND = function() end,
  BROADCAST_DELIVER = function() end,
  UNREGISTER = function() end,
  UNREGISTER_OK = function() end,
  ERROR = function() end
}


---@class client
local cl = {}

---@param msg string
function cl:pack(msg, b)
  local ret = "<" .. tostring(self.nickname) .. "><" .. tostring(self.ip) .. "><" .. tostring(self.port) .. ">"
  if b == false then return ret end
  return tostring(msg) .. ret
end

function cl:upack(fmt_msg, b)
  local reg = "<([%w]+%$)><(%d+%.%d+%.%d+%.%d+)><([%d]+)>"
  if b == false then
    return fmt_msg:match(reg)
  else
    reg = "([%u]+)" .. reg
    print(reg)
    return fmt_msg:match(reg)
  end
end

function cl:connect(dst_ip, port)
  print "Creating TCP Socket"
  local client = assert(socket.tcp())
  print("Connecting to " .. dst_ip .. " : " .. port)
  local success, err = client:connect(dst_ip, port)
  if not success then
    print('Connection Error: ' .. tostring(err))
    return nil
    -- os.exit()
  end
  return client
end

function cl:send_close(client)
  print "SENDING CLOSE Cmd..."
  client:send("CLOSE")

  local close_recv, cl_err = client:receive()

  if not close_recv then print "CLOSING ACCEPTED..." end
end

function cl:send(msg_fin, client)
  print("Sending: " .. tostring(msg_fin))
  client:send(msg_fin .. "\n")

  local resp, err_rec, partial = client:receive()
  if not err_rec then
    print('Server responded: ' .. tostring(resp))
    cl:send_close(client)
  elseif err_rec == "closed" or err_rec == "timeout" then
    print(tostring(err_rec))
  end
  client:close()
end

function cl:request(dst_ip, port, msg)
  local client = cl:connect(dst_ip, port)
  if client then
    local sockname, sock_port = client:getsockname()
    if sockname and sock_port then self.ip, self.port = sockname, tonumber(sock_port) end
    local msg_fin = msg

    cl:send(msg_fin,client)
  end
end

---@param nickname string
---@param ip string
---@param port integer
---@return client
local create_client = function(nickname, ip, port)
  return setmetatable({
      nickname = nickname or "User123$",
      ip = ip or "127.0.0.1",
      port = port or 5000,
    },
    {
      __index = cl
    })
end


cmds.REGISTER = create_client
cmds.USER_LIST_BEGIN = function(v,b) return cl:upack(v,b) end
return cmds

