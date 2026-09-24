-- Set leader key before lazy.nvim loads
vim.g.mapleader = " "
vim.g.maplocalleader = "\\"

-- Load general Neovim options
require("config.options")

-- Bootstrap & setup lazy.nvim (imports all plugins from lua/plugins/)
require("config.lazy")

-- Load keymaps & welcome message after startup
vim.schedule(function()
	require("config.keymaps")

	local ok, notify = pcall(require, "notify")
	if ok then
		vim.notify = notify
		vim.notify("Have a nice day, i4104!", "info", { title = "Welcome Back" })
	end
end)
