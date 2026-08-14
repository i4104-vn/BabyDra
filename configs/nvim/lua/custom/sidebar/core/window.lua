local M = {}
local config = require("custom.sidebar.config")

M.win = nil

function M.is_valid()
	return M.win and vim.api.nvim_win_is_valid(M.win)
end

function M.open()
	if M.is_valid() then
		vim.api.nvim_set_current_win(M.win)
		return M.win
	end

	local width = config.options.width or 54
	local height = math.max(15, vim.o.lines - 3)

	local row = 0
	local col = math.max(0, vim.o.columns - width - 2)

	local main_buf = vim.api.nvim_create_buf(false, true)
	M.win = vim.api.nvim_open_win(main_buf, true, {
		relative = "editor",
		width = width,
		height = height,
		row = row,
		col = col,
		style = "minimal",
		border = config.options.border or "rounded",
		title = " AGENT & CHANGES ",
		title_pos = "center",
	})

	vim.wo[M.win].number = false
	vim.wo[M.win].relativenumber = false
	vim.wo[M.win].signcolumn = "no"
	vim.wo[M.win].winfixwidth = true

	return M.win
end

function M.close()
	if M.win and vim.api.nvim_win_is_valid(M.win) then
		pcall(vim.api.nvim_win_close, M.win, true)
		M.win = nil
	end
end

return M
