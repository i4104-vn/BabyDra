local M = {}
local parser = require("custom.keymap_viewer.parser")
local ui = require("custom.keymap_viewer.ui")
local config = require("custom.keymap_viewer.config")

function M.open()
	local keymaps = parser.parse_keymaps()
	local opts = config.options
	local width = math.min(opts.max_width, math.floor(vim.o.columns * opts.width_ratio))
	local lines = ui.render_lines(keymaps, width)
	ui.create_window(lines)
end

return M
