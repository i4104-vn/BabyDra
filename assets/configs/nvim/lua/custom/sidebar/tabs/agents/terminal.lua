local M = {}
local config = require("custom.sidebar.config")

M.term_buf = nil
M.launcher_buf = nil
M.selected_choice = 1 -- 1: Start freebuff, 2: Close sidebar

local ns_id = vim.api.nvim_create_namespace("sidebar_launcher_choices")

function M.is_running()
	if not M.term_buf or not vim.api.nvim_buf_is_valid(M.term_buf) then
		return false
	end
	local ok, job_id = pcall(function()
		return vim.b[M.term_buf].terminal_job_id
	end)
	return ok and job_id and job_id > 0
end

function M.render_launcher(win)
	if not M.launcher_buf or not vim.api.nvim_buf_is_valid(M.launcher_buf) then
		return
	end

	local c1_prefix = (M.selected_choice == 1) and " ▶ [1]" or "   [1]"
	local c2_prefix = (M.selected_choice == 2) and " ▶ [2]" or "   [2]"

	local lines = {
		" ", 
		" Freebuff",
		" ──────────────",
		"",
		c1_prefix .. " Freebuff agent",
		c2_prefix .. " Close sidebar",
		"",
		" ───────────────",
		" [j/k/Arrow]: Move ",
		" [Enter/Space]: Chooose",
	}

	vim.bo[M.launcher_buf].modifiable = true
	vim.api.nvim_buf_set_lines(M.launcher_buf, 0, -1, false, lines)
	vim.bo[M.launcher_buf].modifiable = false

	vim.api.nvim_buf_clear_namespace(M.launcher_buf, ns_id, 0, -1)

	vim.api.nvim_set_hl(0, "LauncherChoiceActive", { fg = "#7dc4e4", bg = "#2d3f66", bold = true, default = false })
	vim.api.nvim_set_hl(0, "LauncherChoiceInactive", { fg = "#a9b1d6", default = false })

	-- Line index 4 is Choice 1, Line index 5 is Choice 2
	local hl1 = (M.selected_choice == 1) and "LauncherChoiceActive" or "LauncherChoiceInactive"
	vim.api.nvim_buf_set_extmark(M.launcher_buf, ns_id, 4, 0, {
		line_hl_group = hl1,
		hl_eol = true,
	})

	local hl2 = (M.selected_choice == 2) and "LauncherChoiceActive" or "LauncherChoiceInactive"
	vim.api.nvim_buf_set_extmark(M.launcher_buf, ns_id, 5, 0, {
		line_hl_group = hl2,
		hl_eol = true,
	})
end

function M.execute_choice(win)
	if M.selected_choice == 1 then
		M.start_agent(win)
	else
		local window = require("custom.sidebar.core.window")
		window.close()
	end
end

function M.get_or_create_launcher_buffer(win)
	if M.launcher_buf and vim.api.nvim_buf_is_valid(M.launcher_buf) then
		M.render_launcher(win)
		return M.launcher_buf
	end

	M.launcher_buf = vim.api.nvim_create_buf(false, true)
	vim.bo[M.launcher_buf].filetype = "agents_launcher"
	vim.bo[M.launcher_buf].buftype = "nofile"

	M.selected_choice = 1
	M.render_launcher(win)

	local opts = { noremap = true, silent = true, buffer = M.launcher_buf }

	-- Navigation keymaps
	vim.keymap.set("n", "j", function()
		M.selected_choice = 2
		M.render_launcher(win)
	end, opts)

	vim.keymap.set("n", "<Down>", function()
		M.selected_choice = 2
		M.render_launcher(win)
	end, opts)

	vim.keymap.set("n", "k", function()
		M.selected_choice = 1
		M.render_launcher(win)
	end, opts)

	vim.keymap.set("n", "<Up>", function()
		M.selected_choice = 1
		M.render_launcher(win)
	end, opts)

	-- Selection keymaps
	local confirm_fn = function()
		M.execute_choice(win)
	end

	vim.keymap.set("n", "<CR>", confirm_fn, opts)
	vim.keymap.set("n", "<Space>", confirm_fn, opts)

	-- Direct 1 / 2 shortcuts
	vim.keymap.set("n", "1", function()
		M.selected_choice = 1
		M.execute_choice(win)
	end, opts)

	vim.keymap.set("n", "2", function()
		M.selected_choice = 2
		M.execute_choice(win)
	end, opts)

	-- Mouse click selection
	vim.keymap.set("n", "<LeftMouse>", function()
		local mouse_pos = vim.fn.getmousepos()
		if mouse_pos.winid == win then
			if mouse_pos.line == 5 then
				M.selected_choice = 1
				M.execute_choice(win)
			elseif mouse_pos.line == 6 then
				M.selected_choice = 2
				M.execute_choice(win)
			end
		end
	end, opts)

	return M.launcher_buf
end

function M.start_agent(win)
	if not (win and vim.api.nvim_win_is_valid(win)) then
		local window = require("custom.sidebar.core.window")
		if window.is_valid() then
			win = window.win
		end
	end

	if M.is_running() then
		if win and vim.api.nvim_win_is_valid(win) then
			vim.api.nvim_set_current_win(win)
			vim.api.nvim_win_set_buf(win, M.term_buf)
		end
		return
	end

	if M.term_buf and vim.api.nvim_buf_is_valid(M.term_buf) then
		pcall(vim.api.nvim_buf_delete, M.term_buf, { force = true })
	end

	M.term_buf = vim.api.nvim_create_buf(false, true)
	vim.bo[M.term_buf].filetype = "agents_terminal"

	if win and vim.api.nvim_win_is_valid(win) then
		vim.api.nvim_set_current_win(win)
		vim.api.nvim_win_set_buf(win, M.term_buf)
	end

	vim.api.nvim_buf_call(M.term_buf, function()
		vim.bo[M.term_buf].modified = false
		vim.fn.termopen(config.options.agents_cmd or "freebuff")
	end)
	vim.cmd("startinsert")
end

function M.show(win)
	if not (win and vim.api.nvim_win_is_valid(win)) then
		local window = require("custom.sidebar.core.window")
		if window.is_valid() then
			win = window.win
		end
	end

	if M.is_running() then
		if win and vim.api.nvim_win_is_valid(win) then
			vim.api.nvim_set_current_win(win)
			vim.api.nvim_win_set_buf(win, M.term_buf)
		end
	else
		local l_buf = M.get_or_create_launcher_buffer(win)
		if win and vim.api.nvim_win_is_valid(win) then
			vim.api.nvim_win_set_buf(win, l_buf)
		end
	end
end

-- Backward compatibility alias
function M.start(win)
	M.show(win)
end

return M
