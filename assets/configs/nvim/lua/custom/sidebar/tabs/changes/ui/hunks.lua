local M = {}
local highlights = require("custom.sidebar.tabs.changes.ui.highlights")

function M.render(sidebar_buf, sidebar_win, hunks, decisions, cur_hunk)
	if not sidebar_buf or not vim.api.nvim_buf_is_valid(sidebar_buf) then return {}, {} end

	highlights.setup()
	vim.bo[sidebar_buf].modifiable = true

	local lines = {}
	local hunk_to_sidebar_line = {}
	local sidebar_line_to_hunk = {}

	for i, h in ipairs(hunks) do
		local d = decisions[i]
		local status
		if d == "old" then status = "[KEPT OLD]"
		elseif d == "new" then status = "[KEPT NEW]"
		else status = "[ ? ]" end

		local prefix = (i == cur_hunk) and " ▶ " or "   "
		local header_text = string.format("%sHunk %d (Line %d)  %s", prefix, i, h.orig_line or 1, status)

		table.insert(lines, header_text)
		local s_line = #lines
		hunk_to_sidebar_line[i] = s_line
		sidebar_line_to_hunk[s_line] = i

		local old_sub = ""
		for _, l in ipairs(h.old_lines or {}) do
			local txt = vim.trim(l or "")
			if txt ~= "" then old_sub = txt; break end
		end

		local new_sub = ""
		for _, l in ipairs(h.new_lines or {}) do
			local txt = vim.trim(l or "")
			if txt ~= "" then new_sub = txt; break end
		end

		if old_sub ~= "" then
			if #old_sub > 24 then old_sub = old_sub:sub(1, 22) .. ".." end
			table.insert(lines, "     - " .. old_sub)
			sidebar_line_to_hunk[#lines] = i
		end
		if new_sub ~= "" then
			if #new_sub > 24 then new_sub = new_sub:sub(1, 22) .. ".." end
			table.insert(lines, "     + " .. new_sub)
			sidebar_line_to_hunk[#lines] = i
		end
		table.insert(lines, "")
	end

	vim.api.nvim_buf_set_lines(sidebar_buf, 0, -1, false, lines)
	vim.bo[sidebar_buf].modifiable = false
	vim.bo[sidebar_buf].filetype = "sidebar_changes_list"
	vim.bo[sidebar_buf].buftype = "nofile"

	vim.api.nvim_buf_clear_namespace(sidebar_buf, highlights.ns_side_id, 0, -1)

	for i in ipairs(hunks) do
		local line_num = hunk_to_sidebar_line[i]
		if line_num then
			local hl = (i == cur_hunk) and "SidebarHunkActive" or (decisions[i] and "SidebarHunkResolved" or "SidebarHunkPending")
			vim.api.nvim_buf_set_extmark(sidebar_buf, highlights.ns_side_id, line_num - 1, 0, {
				line_hl_group = hl,
				hl_eol = true,
			})
		end
	end

	if sidebar_win and vim.api.nvim_win_is_valid(sidebar_win) then
		local s_line = hunk_to_sidebar_line[cur_hunk]
		if s_line then pcall(vim.api.nvim_win_set_cursor, sidebar_win, { s_line, 0 }) end
	end

	return hunk_to_sidebar_line, sidebar_line_to_hunk
end

return M
