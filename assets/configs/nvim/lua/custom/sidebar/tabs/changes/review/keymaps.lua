local M = {}
local config = require("custom.sidebar.config")

function M.bind_all(buf, sidebar_buf, shortcut_buf, main_win, sidebar_win, callbacks)
	local k = config.options.diff_keymaps

	local toggle_focus = function()
		local cur_win = vim.api.nvim_get_current_win()
		if cur_win == main_win then
			if sidebar_win and vim.api.nvim_win_is_valid(sidebar_win) then
				vim.api.nvim_set_current_win(sidebar_win)
			end
		else
			if main_win and vim.api.nvim_win_is_valid(main_win) then
				vim.api.nvim_set_current_win(main_win)
			end
		end
	end

	-- 1. Main Code Window Keymaps
	if buf and vim.api.nvim_buf_is_valid(buf) then
		local opts = { noremap = true, silent = true, buffer = buf }

		if k.choose_old then vim.keymap.set("n", k.choose_old, function() callbacks.decide_current("old") end, opts) end
		if k.choose_old_num then vim.keymap.set("n", k.choose_old_num, function() callbacks.decide_current("old") end, opts) end
		if k.choose_new then vim.keymap.set("n", k.choose_new, function() callbacks.decide_current("new") end, opts) end
		if k.choose_new_num then vim.keymap.set("n", k.choose_new_num, function() callbacks.decide_current("new") end, opts) end

		-- Keep All Hunks
		vim.keymap.set("n", "kao", function() callbacks.decide_all("old") end, opts)
		vim.keymap.set("n", "kan", function() callbacks.decide_all("new") end, opts)
		vim.keymap.set("n", "ka", function() callbacks.decide_all("new") end, opts)

		-- Tab & Shift-Tab to jump between hunks
		vim.keymap.set("n", "<Tab>", callbacks.jump_next, opts)
		vim.keymap.set("n", "<S-Tab>", callbacks.jump_prev, opts)

		-- Quick jump shortcuts (Next / Prev Hunk)
		for _, key in ipairs({ "]c", "]g", "]d", "]h" }) do
			vim.keymap.set("n", key, callbacks.jump_next, opts)
		end
		for _, key in ipairs({ "[c", "[g", "[d", "[h" }) do
			vim.keymap.set("n", key, callbacks.jump_prev, opts)
		end

		vim.keymap.set("n", "gg", callbacks.jump_first, opts)
		vim.keymap.set("n", "G", callbacks.jump_last, opts)

		vim.keymap.set("n", "u", function() callbacks.decide_current(nil) end, opts)

		-- Switch focus between Code window and Hunks panel
		for _, key in ipairs({ "gw", "<C-w>", "<C-w>w", "<M-Tab>" }) do
			vim.keymap.set("n", key, toggle_focus, opts)
		end

		vim.keymap.set("n", "q", callbacks.close_and_focus, opts)
		vim.keymap.set("n", "<Esc>", callbacks.close_and_focus, opts)
	end

	-- 2. Sidebar Hunks Window Keymaps
	if sidebar_buf and vim.api.nvim_buf_is_valid(sidebar_buf) then
		local opts = { noremap = true, silent = true, buffer = sidebar_buf }

		local jump_and_focus_code = function()
			callbacks.jump_to_hunk(callbacks.get_cur_hunk())
			if main_win and vim.api.nvim_win_is_valid(main_win) then
				vim.api.nvim_set_current_win(main_win)
			end
		end

		-- Tab & Shift-Tab & Arrows to jump between hunks
		vim.keymap.set("n", "<Tab>", callbacks.jump_next, opts)
		vim.keymap.set("n", "<S-Tab>", callbacks.jump_prev, opts)
		vim.keymap.set("n", "<Down>", callbacks.jump_next, opts)
		vim.keymap.set("n", "<Up>", callbacks.jump_prev, opts)

		for _, key in ipairs({ "]c", "]g", "]d", "]h" }) do
			vim.keymap.set("n", key, callbacks.jump_next, opts)
		end

		for _, key in ipairs({ "[c", "[g", "[d", "[h" }) do
			vim.keymap.set("n", key, callbacks.jump_prev, opts)
		end

		vim.keymap.set("n", "gg", callbacks.jump_first, opts)
		vim.keymap.set("n", "G", callbacks.jump_last, opts)

		vim.keymap.set("n", "<CR>", jump_and_focus_code, opts)
		vim.keymap.set("n", "<Space>", jump_and_focus_code, opts)

		if k.choose_old then vim.keymap.set("n", k.choose_old, function() callbacks.decide_current("old") end, opts) end
		if k.choose_old_num then vim.keymap.set("n", k.choose_old_num, function() callbacks.decide_current("old") end, opts) end
		if k.choose_new then vim.keymap.set("n", k.choose_new, function() callbacks.decide_current("new") end, opts) end
		if k.choose_new_num then vim.keymap.set("n", k.choose_new_num, function() callbacks.decide_current("new") end, opts) end

		-- Keep All Hunks
		vim.keymap.set("n", "kao", function() callbacks.decide_all("old") end, opts)
		vim.keymap.set("n", "kan", function() callbacks.decide_all("new") end, opts)
		vim.keymap.set("n", "ka", function() callbacks.decide_all("new") end, opts)

		vim.keymap.set("n", "u", function() callbacks.decide_current(nil) end, opts)

		-- Switch focus between Code window and Hunks panel
		for _, key in ipairs({ "gw", "<C-w>", "<C-w>w", "<M-Tab>" }) do
			vim.keymap.set("n", key, toggle_focus, opts)
		end

		vim.keymap.set("n", "q", callbacks.close_and_focus, opts)
		vim.keymap.set("n", "<Esc>", callbacks.close_and_focus, opts)

		vim.keymap.set("n", "<LeftMouse>", function()
			local mouse_pos = vim.fn.getmousepos()
			if mouse_pos.winid == sidebar_win then
				local map = callbacks.get_sidebar_line_to_hunk()
				local idx = map[mouse_pos.line]
				if idx then
					callbacks.jump_to_hunk(idx)
				end
			end
		end, opts)
	end

	-- 3. Shortcut Window Keymaps
	if shortcut_buf and vim.api.nvim_buf_is_valid(shortcut_buf) then
		local opts = { noremap = true, silent = true, buffer = shortcut_buf }

		vim.keymap.set("n", "<Tab>", callbacks.jump_next, opts)
		vim.keymap.set("n", "<S-Tab>", callbacks.jump_prev, opts)

		for _, key in ipairs({ "gw", "<C-w>", "<C-w>w", "<M-Tab>" }) do
			vim.keymap.set("n", key, toggle_focus, opts)
		end

		vim.keymap.set("n", "q", callbacks.close_and_focus, opts)
		vim.keymap.set("n", "<Esc>", callbacks.close_and_focus, opts)
	end
end

return M
