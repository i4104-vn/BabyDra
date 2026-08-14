return {
	{
		"williamboman/mason.nvim",
		dependencies = {
			"williamboman/mason-lspconfig.nvim",
			"neovim/nvim-lspconfig",
			"hrsh7th/nvim-cmp",
			"hrsh7th/cmp-nvim-lsp",
			"hrsh7th/cmp-buffer",
			"hrsh7th/cmp-path",
			"L3MON4D3/LuaSnip",
			"saadparwaiz1/cmp_luasnip",
			"onsails/lspkind-nvim",
		},
		build = ":MasonUpdate",
		config = function()
			-- Mason Setup
			require("mason").setup({
				ui = {
					icons = {
						package_installed = "✓",
						package_pending = "➜",
						package_uninstalled = "✗",
					},
				},
			})

			-- Mason LSPConfig Setup
			local mason_lspconfig = require("mason-lspconfig")
			local servers = {
				"ts_ls",        -- TypeScript/JavaScript (updated from tsserver)
				"html",         -- HTML
				"cssls",        -- CSS/SCSS/LESS
				"intelephense", -- PHP
				"lua_ls",       -- Lua
				"omnisharp",    -- C# (OmniSharp)
				"vue_ls",       -- Vue3
			}

			mason_lspconfig.setup({
				ensure_installed = servers,
			})

			-- LSP Server Setup with cmp capabilities
			local lspconfig = require("lspconfig")
			local cmp_nvim_lsp = require("cmp_nvim_lsp")
			local capabilities = cmp_nvim_lsp.default_capabilities()

			for _, server in ipairs(servers) do
				local opts = { capabilities = capabilities }
				if server == "lua_ls" then
					opts.settings = {
						Lua = {
							diagnostics = {
								globals = { "vim" },
							},
						},
					}
				end

				if vim.lsp and vim.lsp.config then
					vim.lsp.config(server, opts)
					vim.lsp.enable(server)
				else
					lspconfig[server].setup(opts)
				end
			end


			-- nvim-cmp setup
			local cmp = require("cmp")
			local lspkind = require("lspkind")

			cmp.setup({
				snippet = {
					expand = function(args)
						require("luasnip").lsp_expand(args.body)
					end,
				},
				mapping = cmp.mapping.preset.insert({
					["<Up>"] = cmp.mapping.select_prev_item(),
					["<Down>"] = cmp.mapping.select_next_item(),
					["<C-Space>"] = cmp.mapping.complete(),
					["<C-e>"] = cmp.mapping.close(),
					["<CR>"] = cmp.mapping.confirm({ select = true }),
				}),
				formatting = {
					format = lspkind.cmp_format({
						mode = "symbol_text",
						maxwidth = 50,
						ellipsis_char = "...",
						menu = {
							buffer = "[Buffer]",
							nvim_lsp = "[LSP]",
							luasnip = "[Snippet]",
							path = "[Path]",
						},
					}),
				},
				sources = cmp.config.sources({
					{ name = "nvim_lsp" },
					{ name = "luasnip" },
				}, {
					{ name = "buffer" },
					{ name = "path" },
				}),
			})
		end,
	},
}
