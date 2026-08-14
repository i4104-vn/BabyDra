return {
	-- Auto Session
	{
		"rmagatti/auto-session",
		config = function()
			local auto_session = require("auto-session")
			auto_session.setup({
				auto_restore_enabled = false,
				auto_session_suppress_dirs = { "~/", "~/Dev/", "~/Downloads", "~/Documents", "~/Desktop/" },
			})

			local keymap = vim.keymap
			keymap.set("n", "<leader>wr", "<cmd>SessionRestore<CR>", { desc = "Restore session for cwd" })
			keymap.set("n", "<leader>ws", "<cmd>SessionSave<CR>", { desc = "Save session for auto session root dir" })
		end,
	},


	-- Autopairs & Autotag
	{
		"windwp/nvim-autopairs",
		dependencies = { "windwp/nvim-ts-autotag" },
		event = "InsertEnter",
		config = function()
			require("nvim-autopairs").setup({})
			require("nvim-ts-autotag").setup({
				opts = {
					enable_close = true,
					enable_rename = true,
					enable_close_on_slash = false,
				},
				per_filetype = {
					["html"] = {
						enable_close = false,
					},
				},
			})
		end,
	},
}
