local M = {}
local config = require("custom.keymap_viewer.config")

function M.render_lines(keymaps, width)
	local lines = {
		" 🚀 KEYBINDINGS QUICK REFERENCE (Auto-Loaded)",
		" ────────────────────────────────────────────────────────────────────────────────────────",
		string.format("  %-26s %-24s %s", "Shortcut", "Mode", "Description"),
		" ────────────────────────────────────────────────────────────────────────────────────────",
	}

	local current_cat = ""
	for _, km in ipairs(keymaps) do
		if km.category ~= current_cat then
			current_cat = km.category
			table.insert(lines, "")
			table.insert(lines, "  📌 " .. current_cat:upper())
			table.insert(lines, "  " .. string.rep("─", math.max(10, width - 6)))
		end

		local line = string.format("  %-26s %-24s %s", km.lhs, km.mode, km.desc)
		table.insert(lines, line)
	end

	table.insert(lines, "")
	table.insert(lines, " ────────────────────────────────────────────────────────────────────────────────────────")
	table.insert(lines, "  [Press <Esc>, q, or <C-q> to exit window]")

	return lines
end

function M.create_window(lines)
	local opts = config.options
	local buf = vim.api.nvim_create_buf(false, true)
	vim.bo[buf].filetype = "keymap_viewer"
	vim.bo[buf].buftype = "nofile"

	local width = math.min(opts.max_width, math.floor(vim.o.columns * opts.width_ratio))
	local height = math.min(opts.max_height, math.floor(vim.o.lines * opts.height_ratio))
	local row = math.floor((vim.o.lines - height) / 2)
	local col = math.floor((vim.o.columns - width) / 2)

	local win_opts = {
		relative = "editor",
		width = width,
		height = height,
		row = row,
		col = col,
		style = "minimal",
		border = opts.border,
		title = opts.title,
		title_pos = "center",
	}

	vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)
	vim.bo[buf].modifiable = false

	local win = vim.api.nvim_open_win(buf, true, win_opts)

	local close_opts = { noremap = true, silent = true, buffer = buf }
	for _, key in ipairs(opts.close_keys) do
		vim.keymap.set("n", key, function()
			if vim.api.nvim_win_is_valid(win) then
				vim.api.nvim_win_close(win, true)
			end
		end, close_opts)
	end
end

return M
