return {
	{
		"nvim-telescope/telescope.nvim",
		dependencies = { "nvim-lua/plenary.nvim" },
		config = function()
			-- Polyfill nvim-treesitter compatibility for telescope previewer
			package.preload["nvim-treesitter.configs"] = package.preload["nvim-treesitter.configs"] or function()
				return {
					is_enabled = function() return false end,
					get_module = function() return { additional_vim_regex_highlighting = false } end,
				}
			end

			local ok_parsers, parsers = pcall(require, "nvim-treesitter.parsers")
			if ok_parsers and parsers then
				if not parsers.ft_to_lang then
					parsers.ft_to_lang = function(ft)
						return (vim.treesitter.language and vim.treesitter.language.get_lang and vim.treesitter.language.get_lang(ft)) or ft
					end
				end
			end

			require("telescope").setup({
				defaults = {
					preview = {
						treesitter = false,
					},
				},
			})
		end,
	},
}
