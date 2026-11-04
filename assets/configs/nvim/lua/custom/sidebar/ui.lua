local M = {}
local config = require("custom.sidebar.config")
local window = require("custom.sidebar.core.window")
local statusline = require("custom.sidebar.core.statusline")
local terminal = require("custom.sidebar.tabs.agents.terminal")
local changes = require("custom.sidebar.tabs.changes")

M.current_tab = 1

function M.switch_tab(tab_idx)
	if not window.is_valid() then return end
	M.current_tab = tab_idx

	if M.current_tab == 1 then
		terminal.start(window.win)
	else
		local buf = changes.get_or_create_buffer()
		vim.api.nvim_win_set_buf(window.win, buf)
	end

	statusline.update(M.current_tab)
	M.bind_keymaps()
end

function M.toggle_tab()
	if M.current_tab == 1 then
		M.switch_tab(2)
	else
		M.switch_tab(1)
	end
end

function M.bind_keymaps()
	if not window.is_valid() then return end
	local buf = vim.api.nvim_win_get_buf(window.win)
	local opts = { noremap = true, silent = true, buffer = buf }
	local k = config.options.keymaps

	if k.switch_tab then vim.keymap.set({ "n", "t" }, k.switch_tab, function() M.toggle_tab() end, opts) end
	if k.close_sidebar then vim.keymap.set({ "n", "t" }, k.close_sidebar, function() window.close() end, opts) end
	vim.keymap.set({ "n", "t" }, "<A-1>", function() M.switch_tab(1) end, opts)
	vim.keymap.set({ "n", "t" }, "<A-2>", function() M.switch_tab(2) end, opts)
	vim.keymap.set({ "n" }, "1", function() M.switch_tab(1) end, opts)
	vim.keymap.set({ "n" }, "2", function() M.switch_tab(2) end, opts)
end

function M.open()
	window.open()
	M.switch_tab(M.current_tab)
end

function M.close()
	window.close()
end

function M.toggle()
	if window.is_valid() then
		local cur_win = vim.api.nvim_get_current_win()
		if cur_win == window.win then
			M.close()
		else
			vim.api.nvim_set_current_win(window.win)
			M.bind_keymaps()
		end
	else
		M.open()
	end
end

return M
