local M = {}
local ui = require("custom.sidebar.ui")
local config = require("custom.sidebar.config")
local resolver = require("custom.sidebar.tabs.changes.fs.resolver")

function M.toggle()
	ui.toggle()
end

function M.open()
	ui.open()
end

function M.close()
	ui.close()
end

function M.open_agents()
	ui.open()
	ui.switch_tab(1)
end

function M.open_changes()
	ui.open()
	ui.switch_tab(2)
end

function M.toggle_tab()
	ui.toggle_tab()
end

-- Inline Diff Resolution APIs (can be called directly from keymaps.lua or buffer)
function M.choose_old(buf)
	resolver.choose_old(buf or 0)
end

function M.choose_new(buf)
	resolver.choose_new(buf or 0)
end

function M.undo_last_resolution()
	resolver.undo_last_resolution()
end

return M
