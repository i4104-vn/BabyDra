local M = {}

local git_status = require("custom.sidebar.tabs.changes.git.status")
local git_state = require("custom.sidebar.tabs.changes.git.state")
local resolver = require("custom.sidebar.tabs.changes.fs.resolver")
local review = require("custom.sidebar.tabs.changes.review")

M.changes_buf = nil

function M.open_inline_diff(filepath)
	if not filepath or vim.fn.filereadable(filepath) == 0 then return end
	review.open(filepath)
end

function M.close_diff_view()
	review.close()
	M.focus_changes()
end

function M.finish_resolution()
	M.close_diff_view()
end

function M.after_undo(filepath)
	M.focus_changes()
	if filepath then
		M.open_inline_diff(filepath)
	end
end

function M.focus_changes()
	local sidebar_win = require("custom.sidebar.core.window").win
	if sidebar_win and vim.api.nvim_win_is_valid(sidebar_win) then
		vim.api.nvim_set_current_win(sidebar_win)
	end
	if M.changes_buf and vim.api.nvim_buf_is_valid(M.changes_buf) then
		M.render_buffer(M.changes_buf)
	end
end

local ns_changes_id = vim.api.nvim_create_namespace("changes_selection_hl")
M.selected_index = 1

function M.render_buffer(buf)
	vim.bo[buf].modifiable = true
	local all_files = git_status.get_modified_files()

	local files = {}
	for _, f in ipairs(all_files) do
		if not git_state.is_resolved(f.file) then
			table.insert(files, f)
		end
	end

	M.rendered_files = {}
	M.line_to_index = {}

	if #files > 0 then
		if M.selected_index > #files then M.selected_index = #files end
		if M.selected_index < 1 then M.selected_index = 1 end
	else
		M.selected_index = 1
	end

	local lines = {
		"",
		" 📂 MODIFIED / CREATED FILES (" .. #files .. ")",
		" ──────────────",
	}

	if #files == 0 then
		table.insert(lines, "")
		table.insert(lines, "  ✓ No modified files")
	else
		table.insert(lines, "")
		for i, f in ipairs(files) do
			local name = vim.fn.fnamemodify(f.file, ":t")
			local label = f.label or "[M]"
			local prefix = (i == M.selected_index) and string.format(" ▶ %s", label) or string.format("   %s", label)
			table.insert(lines, string.format("%s %s", prefix, name))
			local line_idx = #lines
			M.rendered_files[line_idx] = f.file
			M.line_to_index[line_idx] = i
		end
	end

	table.insert(lines, "")
	table.insert(lines, " ──────────────")
	table.insert(lines, " [j/k/Arrow]: Move")
	table.insert(lines, " [Enter/Space/o]: Diff View")
	table.insert(lines, " [r]: Refresh List")

	vim.api.nvim_buf_set_lines(buf, 0, -1, false, lines)
	vim.bo[buf].modifiable = false
	vim.bo[buf].filetype = "sidebar_changes"
	vim.bo[buf].buftype = "nofile"

	vim.api.nvim_buf_clear_namespace(buf, ns_changes_id, 0, -1)
	vim.api.nvim_set_hl(0, "ChangesChoiceActive", { fg = "#7dc4e4", bg = "#2d3f66", bold = true, default = false })
	vim.api.nvim_set_hl(0, "ChangesChoiceInactive", { fg = "#a9b1d6", default = false })

	for line_idx, i in pairs(M.line_to_index) do
		local hl = (i == M.selected_index) and "ChangesChoiceActive" or "ChangesChoiceInactive"
		vim.api.nvim_buf_set_extmark(buf, ns_changes_id, line_idx - 1, 0, {
			line_hl_group = hl,
			hl_eol = true,
		})
	end

	local opts = { noremap = true, silent = true, buffer = buf }

	vim.keymap.set("n", "j", function()
		if #files > 0 then
			M.selected_index = math.min(#files, M.selected_index + 1)
			M.render_buffer(buf)
		end
	end, opts)

	vim.keymap.set("n", "<Down>", function()
		if #files > 0 then
			M.selected_index = math.min(#files, M.selected_index + 1)
			M.render_buffer(buf)
		end
	end, opts)

	vim.keymap.set("n", "k", function()
		if #files > 0 then
			M.selected_index = math.max(1, M.selected_index - 1)
			M.render_buffer(buf)
		end
	end, opts)

	vim.keymap.set("n", "<Up>", function()
		if #files > 0 then
			M.selected_index = math.max(1, M.selected_index - 1)
			M.render_buffer(buf)
		end
	end, opts)

	local open_diff = function()
		if #files > 0 and files[M.selected_index] then
			M.open_inline_diff(files[M.selected_index].file)
		else
			local cursor = vim.api.nvim_win_get_cursor(0)
			local line_num = cursor[1]
			local filepath = M.rendered_files and M.rendered_files[line_num]
			if filepath then
				M.open_inline_diff(filepath)
			end
		end
	end

	vim.keymap.set("n", "<CR>", open_diff, opts)
	vim.keymap.set("n", "<Space>", open_diff, opts)
	vim.keymap.set("n", "o", open_diff, opts)
	vim.keymap.set("n", "r", function() M.render_buffer(buf) end, opts)

	vim.keymap.set("n", "u", function() resolver.undo_last_resolution() end, opts)
	vim.keymap.set("n", "<C-z>", function() resolver.undo_last_resolution() end, opts)

	vim.keymap.set("n", "<LeftMouse>", function()
		local mouse_pos = vim.fn.getmousepos()
		if mouse_pos.winid == vim.api.nvim_get_current_win() then
			local idx = M.line_to_index[mouse_pos.line]
			if idx then
				M.selected_index = idx
				M.render_buffer(buf)
				if files[idx] then
					M.open_inline_diff(files[idx].file)
				end
			end
		end
	end, opts)
end

function M.get_or_create_buffer()
	if M.changes_buf and vim.api.nvim_buf_is_valid(M.changes_buf) then
		M.render_buffer(M.changes_buf)
		return M.changes_buf
	end

	M.changes_buf = vim.api.nvim_create_buf(false, true)
	M.render_buffer(M.changes_buf)
	return M.changes_buf
end

return M
