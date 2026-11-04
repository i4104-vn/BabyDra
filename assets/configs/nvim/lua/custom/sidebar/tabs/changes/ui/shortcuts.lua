local M = {}

function M.render(shortcut_buf)
	if not shortcut_buf or not vim.api.nvim_buf_is_valid(shortcut_buf) then return end

	vim.bo[shortcut_buf].modifiable = true
	local lines = {
		" [Tab / S-Tab] : Next / Prev Hunk",
		" [1 / ko]       : Keep OLD hunk",
		" [2 / kn]       : Keep NEW hunk",
		" [kao / kan]    : Keep ALL Old/New",
		" [u]            : Undo hunk choice",
		" [gg / G]       : First / Last hunk",
		" [gw / C-w]     : Switch Code/List",
		" [q / Esc]      : Close review",
	}
	vim.api.nvim_buf_set_lines(shortcut_buf, 0, -1, false, lines)
	vim.bo[shortcut_buf].modifiable = false
	vim.bo[shortcut_buf].filetype = "sidebar_shortcuts_list"
	vim.bo[shortcut_buf].buftype = "nofile"
end

return M
