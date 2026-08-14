local M = {}
local window = require("custom.sidebar.core.window")

function M.update(current_tab)
	if not window.win or not vim.api.nvim_win_is_valid(window.win) then return end

	local tab1_label = " 🤖 [1: Freebuff] "
	local tab2_label = " 📂 [2: Changes] "

	if current_tab == 1 then
		vim.wo[window.win].winbar = "%#TabLineSel#" .. tab1_label .. "%#TabLine#" .. tab2_label
	else
		vim.wo[window.win].winbar = "%#TabLine#" .. tab1_label .. "%#TabLineSel#" .. tab2_label
	end
end

return M
