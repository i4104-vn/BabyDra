local M = {}

function M.create_windows(filepath, buf, sidebar_buf, shortcut_buf, hunks_count)
	local total_w = math.max(65, math.min(vim.o.columns - 4, 160))
	local total_h = math.max(16, math.min(vim.o.lines - 4, 38))

	local side_w = 38
	local main_w = total_w - side_w - 2
	if main_w < 30 then main_w = 30 end

	local shortcut_h = 10
	local hunk_h = total_h - shortcut_h - 2
	if hunk_h < 5 then hunk_h = 5 end

	local row = math.max(1, math.floor((vim.o.lines - total_h) / 2))
	local col_main = math.max(1, math.floor((vim.o.columns - total_w) / 2))
	local col_side = col_main + main_w + 2
	local row_shortcut = row + hunk_h + 2

	local filename = vim.fn.fnamemodify(filepath, ":t")

	local main_win = vim.api.nvim_open_win(buf, true, {
		relative = "editor",
		width = main_w,
		height = total_h,
		row = row,
		col = col_main,
		style = "minimal",
		border = "rounded",
		title = " 📄 CODE: " .. filename .. " (READ-ONLY) ",
		title_pos = "left",
	})

	vim.wo[main_win].number = true
	vim.wo[main_win].relativenumber = false
	vim.wo[main_win].signcolumn = "no"
	vim.wo[main_win].cursorline = true
	vim.wo[main_win].winfixwidth = true
	vim.wo[main_win].winhl = "NormalFloat:NormalFloat"

	local sidebar_win = vim.api.nvim_open_win(sidebar_buf, false, {
		relative = "editor",
		width = side_w,
		height = hunk_h,
		row = row,
		col = col_side,
		style = "minimal",
		border = "rounded",
		title = " 📌 CHANGES (" .. hunks_count .. ") ",
		title_pos = "center",
	})

	vim.wo[sidebar_win].number = false
	vim.wo[sidebar_win].relativenumber = false
	vim.wo[sidebar_win].signcolumn = "no"
	vim.wo[sidebar_win].cursorline = true
	vim.wo[sidebar_win].winfixwidth = true
	vim.wo[sidebar_win].winhl = "NormalFloat:NormalFloat"

	local shortcut_win = vim.api.nvim_open_win(shortcut_buf, false, {
		relative = "editor",
		width = side_w,
		height = shortcut_h,
		row = row_shortcut,
		col = col_side,
		style = "minimal",
		border = "rounded",
		title = " ⌨ SHORTCUTS ",
		title_pos = "center",
	})

	vim.wo[shortcut_win].number = false
	vim.wo[shortcut_win].relativenumber = false
	vim.wo[shortcut_win].signcolumn = "no"
	vim.wo[shortcut_win].cursorline = false
	vim.wo[shortcut_win].winfixwidth = true
	vim.wo[shortcut_win].winhl = "NormalFloat:NormalFloat"

	return main_win, sidebar_win, shortcut_win
end

return M
