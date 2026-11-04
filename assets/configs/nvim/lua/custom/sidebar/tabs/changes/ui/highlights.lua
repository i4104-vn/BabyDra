local M = {}

M.ns_id = vim.api.nvim_create_namespace("floating_review_hl")
M.ns_side_id = vim.api.nvim_create_namespace("floating_review_side_hl")

function M.setup()
	vim.api.nvim_set_hl(0, "DiffOldHeader", { fg = "#ff5555", bg = "#441c24", bold = true, default = false })
	vim.api.nvim_set_hl(0, "DiffDivider", { fg = "#ffb86c", bg = "#382a1c", bold = true, default = false })
	vim.api.nvim_set_hl(0, "DiffNewHeader", { fg = "#50fa7b", bg = "#1c4428", bold = true, default = false })
	vim.api.nvim_set_hl(0, "DiffOldLine", { fg = "#ff9999", bg = "#3c181e", default = false })
	vim.api.nvim_set_hl(0, "DiffNewLine", { fg = "#99ff99", bg = "#183c20", default = false })
	vim.api.nvim_set_hl(0, "DiffCurrentHunk", { fg = "#000000", bg = "#8fbcbb", bold = true, default = false })

	vim.api.nvim_set_hl(0, "SidebarHunkActive", { fg = "#7dc4e4", bg = "#2d3f66", bold = true, default = false })
	vim.api.nvim_set_hl(0, "SidebarHunkResolved", { fg = "#50fa7b", bold = true, default = false })
	vim.api.nvim_set_hl(0, "SidebarHunkPending", { fg = "#a9b1d6", default = false })
end

function M.apply_main_hl(buf, hunk_line_map, cur_hunk)
	if not buf or not vim.api.nvim_buf_is_valid(buf) or not hunk_line_map then return end
	M.setup()
	vim.api.nvim_buf_clear_namespace(buf, M.ns_id, 0, -1)

	for idx, info in pairs(hunk_line_map) do
		local is_active = (idx == cur_hunk)
		if not info.is_resolved then
			local old_hl = is_active and "DiffCurrentHunk" or "DiffOldHeader"
			local div_hl = is_active and "DiffCurrentHunk" or "DiffDivider"
			local new_hl = is_active and "DiffCurrentHunk" or "DiffNewHeader"

			vim.api.nvim_buf_set_extmark(buf, M.ns_id, info.old_hdr - 1, 0, { line_hl_group = old_hl, hl_eol = true })
			vim.api.nvim_buf_set_extmark(buf, M.ns_id, info.div_hdr - 1, 0, { line_hl_group = div_hl, hl_eol = true })
			vim.api.nvim_buf_set_extmark(buf, M.ns_id, info.new_hdr - 1, 0, { line_hl_group = new_hl, hl_eol = true })

			for l = info.old_hdr + 1, info.div_hdr - 1 do
				vim.api.nvim_buf_set_extmark(buf, M.ns_id, l - 1, 0, { line_hl_group = "DiffOldLine", hl_eol = true })
			end
			for l = info.div_hdr + 1, info.new_hdr - 1 do
				vim.api.nvim_buf_set_extmark(buf, M.ns_id, l - 1, 0, { line_hl_group = "DiffNewLine", hl_eol = true })
			end
		else
			local hl = (info.choice == "old") and "DiffOldLine" or "DiffNewLine"
			if is_active then
				hl = "SidebarHunkActive"
			end
			for l = info.start_line, info.end_line do
				vim.api.nvim_buf_set_extmark(buf, M.ns_id, l - 1, 0, { line_hl_group = hl, hl_eol = true })
			end
		end
	end
end

return M
