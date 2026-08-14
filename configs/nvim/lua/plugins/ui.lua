return {
	-- Tokyonight Theme
	{
		"folke/tokyonight.nvim",
		lazy = false,
		priority = 1000,
		config = function()
			vim.cmd([[colorscheme tokyonight-night]])
		end,
	},

	-- Web Devicons
	{ "nvim-tree/nvim-web-devicons", lazy = true },

	-- Nvim Tree
	{
		"nvim-tree/nvim-tree.lua",
		version = "*",
		lazy = false,
		dependencies = { "nvim-tree/nvim-web-devicons" },
		config = function()
			local api = require("nvim-tree.api")
			require("nvim-tree").setup({
				update_focused_file = {
					enable = true,
					update_root = false,
				},
				view = {
					width = 35,
					side = "left",
				},
				actions = {
					open_file = {
						quit_on_open = false,
						resize_window = true,
					},
				},
			})

			vim.api.nvim_create_autocmd("BufEnter", {
				nested = true,
				callback = function(args)
					local ft = vim.bo[args.buf].filetype
					local bt = vim.bo[args.buf].buftype
					local name = vim.api.nvim_buf_get_name(args.buf)

					if bt == "" and ft ~= "NvimTree" and ft ~= "alpha" and ft ~= "sidebar_changes" and name ~= "" and vim.fn.filereadable(name) == 1 then
						pcall(api.tree.find_file, { open = true, focus = false })
					end
				end,
			})
		end,
	},

	-- Lualine
	{
		"nvim-lualine/lualine.nvim",
		dependencies = { "nvim-tree/nvim-web-devicons" },
		config = function()
			require("lualine").setup({
				options = {
					icons_enabled = true,
					theme = "auto",
					component_separators = { left = "", right = "" },
					section_separators = { left = "", right = "" },
					disabled_filetypes = {
						statusline = { "NvimTree", "sidebar_changes", "alpha", "dashboard" },
						winbar = { "NvimTree", "sidebar_changes", "alpha", "dashboard" },
					},
					ignore_focus = {},
					always_divide_middle = true,
					globalstatus = true,
					refresh = { statusline = 1000, tabline = 1000, winbar = 1000 },
				},
				sections = {
					lualine_a = { "mode" },
					lualine_b = { "branch", "diff", "diagnostics" },
					lualine_c = { "filename" },
					lualine_x = { "encoding", "fileformat", "filetype" },
					lualine_y = { "progress" },
					lualine_z = { "location" },
				},
				inactive_sections = {
					lualine_a = {},
					lualine_b = {},
					lualine_c = { "filename" },
					lualine_x = { "location" },
					lualine_y = {},
					lualine_z = {},
				},
			})
		end,
	},

	-- Dashboard (Alpha)
	{
		"goolord/alpha-nvim",
		lazy = false,
		priority = 1000,
		event = "VimEnter",
		config = function()
			local alpha = require("alpha")
			local dashboard = require("alpha.themes.dashboard")

			dashboard.section.header.val = {
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣶⣿⠟⠉⠉⠻⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣾⠿⠉⠀⠀⠀⠀⠀⠹⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣴⣾",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣴⣾⡿⠛⠉",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣴⣾⡿⠟⠁⠀⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣴⣿⡿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣷⣶⣶⣦⣤⣤⣄⣤⡀⣀⣩⣾⣿⠿⠋⠀⠀⠀⠀⠀⣠",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⠟⠋⠉⠉⠙⣿⣿⣿⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⢀⡾⠁",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⠁⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣴⣿⣿⣿⡟⠁⠀⠀⠀⢀⣴⣿⣿⣿⣿⡏⠋⠀⠀⠀⠀⠀⠀⠀⡞⠋⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣼⣿⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⣿⡏⠀⠀⠀⠀⣶⣿⣿⣿⣿⡿⠉⠀⠀⠀⠀⠀⠀⠀⠀⢸⡯⠤⣤",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣰⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⣿⣿⣿⡏⠀⠀⠀⠀⣼⣿⣿⣿⣿⡟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡾⠁",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣄⣄⣼⣿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢻⡿⠋⠀⠀⠀⠀⣼⣿⣿⣿⡿⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⢷⣄",
				"⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣀⠀⠀⠀⠀⣾⣿⣿⣿⡿⠁⠀⠀⠀⠀⠀⠀⣠⣶⣶⣶⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠿⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻",
				"⠀⠀⠀⠀⠀⠀⢀⣾⠿⠛⢿⣿⣷⣄⡀⣿⠋⠀⠈⠀⠀⠀⠀⠀⠀⢀⣾⡏⠀⢹⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠀⠀⣠⣤⣦⣼⣿⠀⠀⠀⣿⣿⣿⣿⣿⣦⣀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⢀⣀⣀⣀⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣾⣿⣿⣿⢷⣄⠀⠀⠀⠀⠀⠀⠀",
				"⠀⣠⣾⡿⠋⠉⠉⠁⠀⠀⠀⠀⠉⢯⡙⠻⣿⣿⣷⣤⡀⠀⠀⠀⠀⢿⣿⣿⣿⣿⡿⠃⢀⡤⠖⠋⣉⣉⣉⣉⠉⠉⠒⦄⠀⠀⠀⢔⡟⡿⠟⠉⣟⣻⣮⣿⠀⠀⠀⠀⠀⠀⠀⠀",
				"⣾⣿⠋⠀⠀⠀⠀⣀⣀⡀⠀⠀⠀⠀⠙⢦⣄⠉⠻⢿⣿⣷⣦⡀⠀⠈⠙⠛⠛⠋⠀⢰⠟⡇⠀⠀⠈⠻⡿⠛⠁⠀⠀⠀⠈⠳⣄⠀⠸⣧⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀⠀⠀⠀⠀",
				"⣿⡇⠀⠀⠀⣴⠙⣩⣿⣿⣄⠀⠀⠀⠀⡶⢌⡙⠶⣤⡈⠛⠿⣿⣷⣦⣀⠀⠀⠀⠀⡇⠀⢻⣄⠀⠀⣠⢷⠀⠀⠀⠀⠀⡶⠀⠘⡆⠀⠀⠻⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀⠀⠀⠀",
				"⣿⡇⠀⠀⢸⣇⢸⣿⣿⣿⣿⠀⠀⠀⠀⡇⠀⠈⠛⠦⣝⡳⢤⣈⠛⠻⣿⣷⣦⣀⠀⠀⠀⠀⠈⠙⠋⠁⠀⠛⠦⢤⡤⠞⠃⠀⠀⢳⠀⠀⠀⠈⠋⠙⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⣿⡇⠀⠀⠈⢿⣄⣿⣿⣿⠏⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠙⠳⢬⣛⠦⠀⠙⢻⣿⣷⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡞⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⣿⡇⠀⠀⠀⠀⠉⠛⠋⠁⠀⠀⠀⠀⠀⡇⠀⠀⠀⠀⠀⠀⠀⠀⠈⠁⠀⠀⠈⣿⠉⢻⣿⣷⣦⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡼⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⣿⡇⠀⠀⠀⠀⠀⣠⣄⠀⠀⢰⠶⠒⠒⢧⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⢸⡇⢸⡟⢿⣷⣦⣴⣶⣶⣶⣶⣤⣔⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⣿⡇⠀⠀⣤⠀⠀⠿⠿⠁⢀⡿⠀⠀⠀⡄⠈⠙⡷⢦⣄⡀⠀⠀⠀⠀⠀⠀⠀⣿⠀⢸⡇⢸⡇⠀⣿⠙⣿⣿⣉⠉⠙⠿⣿⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⣿⡇⠀⠀⠙⠷⢤⣀⣠⠴⠛⠁⠀⠀⠀⠇⠀⠀⡇⢸⡏⢹⡷⢦⣄⡀⠀⠀⠀⣿⡀⢸⡇⢸⡇⠀⡟⠀⢸⠀⢹⡷⢦⣄⣘⣿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⣿⣿⠢⣤⡀⠀⠀⠀⠀⠀⠀⣠⠾⣿⣿⡷⣤⣀⡇⠸⡇⢸⡇⢸⠉⠙⠳⢦⣄⡻⢿⣾⣧⣸⣧⠀⡇⠀⢹⡀⢸⡇⢤⣈⠙⠻⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⢹⣿⣷⣌⡉⠛⠲⢶⣶⠖⠛⠛⢶⣄⡉⠛⠿⣽⣿⣶⣧⣸⡇⢸⠀⠀⠀⠀⠈⠙⠲⢮⣝⠻⣿⣷⣷⣄⣼⠀⢸⡇⠀⠈⠁⠀⢸⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠈⠙⠻⢿⣷⣶⣤⣉⡻⢶⣄⣀⠈⠙⠳⢦⣈⡉⠻⢿⣿⣷⣾⣦⡀⠀⠀⠀⠀⠀⠀⠈⠙⠲⢭⣛⠿⣿⣷⣼⡇⠀⠀⠀⠀⠈⣿⡇⠀⠀⠀⠀⠀⠀⣀⠀⠀⠀⠀⠀⠀⠀⠀⣠",
				"⠀⠀⠀⠀⠀⠈⠙⠻⢿⣿⣷⣶⣽⣻⡦⠀⠀⠈⠙⠷⣦⣌⡙⠻⢿⣟⣷⣤⣀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠳⢯⣻⡇⠀⠀⠀⠀⠀⢸⣿⠀⣀⠀⠀⠀⠀⠈⠳⣄⡀⠀⠀⢀⣏⡟⠛",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠻⢿⣿⣿⣿⣶⣤⣤⣤⣀⣈⠛⠷⣤⣈⡛⠷⢽⡻⢶⣄⣀⠀⠀⠀⠀⠀⠀⠀⠈⠛⠳⢤⣀⠀⠀⢸⣿⡀⠈⠻⢭⣲⣴⣴⠗⠋⠛⠶⠿⠿⠃⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢈⣿⡿⠛⠉⠙⠛⠛⠻⢷⣦⣄⣩⣿⠶⠖⠛⠛⠛⠛⠛⠛⠿⢷⣶⣦⣄⠀⠀⠀⠀⠉⢻⣶⣿⣿⠇⠀⠀⠀⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣿⠁⠀⠀⠀⠀⠀⠀⠀⣿⣿⠋⠀⠀⠀⠀⢀⣠⡶⠂⠀⠀⠀⠈⠙⠿⣿⣦⡄⠀⠀⣸⣿⠟⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⣴⠏⠁⠀⠀⠀⠀⠀⠀⠀⠀⠈⠛⢿⣶⣄⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀⠀⢸⣧⠀⠀⢀⡾⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⠙⢿⣿⣇⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡿⠿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡿⠦⠠⠋⠀⠀⠀⠀⠀⢀⡶⠂⠀⠀⠀⠀⠀⠀⠧⠤⠄⠙⡿⠿⠦⠤⠤⠤⠤⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀Why⠀bother⠀coding⠀today⠀when⠀you⠀can⠀embrace⠀the⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀art⠀of⠀procrastination⠀and⠀let⠀tomorrow's⠀coffee⠀⠀⠀⠀⠀⠀⠀⠀⠀",
				"⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀do⠀the⠀hard⠀work?⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀",
			}

			dashboard.section.buttons.val = {
				dashboard.button("e", "📄  New File", "<cmd>ene<CR>"),
				dashboard.button("`", "📂  Toggle Sidebar", "<cmd>NvimTreeToggle<CR>"),
				dashboard.button("r", "📜  Restore Session", "<cmd>SessionRestore<CR>"),
				dashboard.button("q", "☕️  Sip coffee, relax!", "<cmd>qa<CR>"),
			}

			alpha.setup(dashboard.opts)
			vim.cmd([[autocmd FileType alpha setlocal nofoldenable]])
		end,
	},

	-- Indent Blankline
	{
		"lukas-reineke/indent-blankline.nvim",
		main = "ibl",
		config = function()
			require("ibl").setup({
				indent = { char = "│" },
				whitespace = { remove_blankline_trail = false },
				scope = { enabled = false },
			})
		end,
	},

	-- Colorizer
	{
		"norcalli/nvim-colorizer.lua",
		config = function()
			require("colorizer").setup()
		end,
	},

	-- Notifications (nvim-notify)
	{
		"rcarriga/nvim-notify",
		config = function()
			local notify = require("notify")
			notify.setup({
				stages = "fade",
				timeout = 3000,
				max_width = 50,
				max_height = 20,
				render = "wrapped-compact",
			})

			vim.cmd([[highlight NotifyINFOIcon guifg=#7dc4e4]])
			vim.cmd([[highlight NotifyINFOTitle guifg=#7dc4e8]])
			vim.cmd([[highlight NotifyINFOBorder guifg=#7dc4e0]])

			vim.cmd([[highlight NotifySUCCESSIcon guifg=#A9FF68]])
			vim.cmd([[highlight NotifySUCCESSTitle guifg=#4CAF50]])
			vim.cmd([[highlight NotifySUCCESSBorder guifg=#3C763D]])

			vim.cmd([[highlight NotifyERRORIcon guifg=#FF6F61]])
			vim.cmd([[highlight NotifyERRORTitle guifg=#F44336]])
			vim.cmd([[highlight NotifyERRORBorder guifg=#C62828]])

			vim.cmd([[highlight NotifyDANGERIcon guifg=#FF6F61]])
			vim.cmd([[highlight NotifyDANGERTitle guifg=#FF5722]])
			vim.cmd([[highlight NotifyDANGERBorder guifg=#D32F2F]])

			vim.cmd([[highlight NotifyWARNIcon guifg=#FFC107]])
			vim.cmd([[highlight NotifyWARNTITLE guifg=#FF9800]])
			vim.cmd([[highlight NotifyWARNBorder guifg=#FFA000]])

			vim.cmd([[highlight NotifyDEBUGIcon guifg=#673AB7]])
			vim.cmd([[highlight NotifyDEBUGTitle guifg=#9C27B0]])
			vim.cmd([[highlight NotifyDEBUGBorder guifg=#6A1B9A]])

			vim.notify = notify
			pcall(function()
				require("telescope").load_extension("notify")
			end)
		end,
	},
}
