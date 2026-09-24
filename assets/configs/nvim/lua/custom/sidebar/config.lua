local M = {}

M.options = {
	width = 50,
	position = "right",
	agents_cmd = "freebuff",

	keymaps = {
		toggle_sidebar = "<A-a>",
		switch_tab = "<Tab>",
		close_sidebar = "<C-q>",
	},

	diff_keymaps = {
		choose_old = "ko",
		choose_old_num = "1",
		choose_new = "kn",
		choose_new_num = "2",
		next_hunk = "]c",
		prev_hunk = "[c",
	},
}

function M.setup(opts)
	if opts then
		M.options = vim.tbl_deep_extend("force", M.options, opts)
	end
end

return M
