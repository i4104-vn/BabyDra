local map = vim.keymap.set

-- Telescope Keymaps
map("n", "<C-,>", "<cmd>Telescope find_files<CR>", { desc = "Find files (Telescope)" })
map("n", "<C-o>", "<cmd>Telescope find_files<CR>", { desc = "Find files (Telescope)" })
map("n", "<C-Tab>", "<cmd>Telescope buffers<CR>", { desc = "List open buffers (Telescope)" })
map("n", "<C-b>", "<cmd>Telescope buffers<CR>", { desc = "List open buffers (Telescope)" })
map("n", "<leader>fk", "<cmd>Telescope keymaps<CR>", { desc = "Search all keymaps (Telescope)" })

-- Keymap Float Viewer (Custom Floating Window)
local keymap_viewer = require("custom.keymap_viewer")
map("n", "<leader>hk", function() keymap_viewer.open() end, { desc = "Open Keymap Float Viewer" })
map("n", "<F1>", function() keymap_viewer.open() end, { desc = "Open Keymap Float Viewer" })

-- Agents & Changes Sidebar (Custom Right Split)
local sidebar = require("custom.sidebar")
map({ "n", "t", "i" }, "<A-a>", function() sidebar.toggle() end, { desc = "Toggle & Focus Agents & Changes Sidebar" })




-- NvimTree Keymaps
map("n", "<C-`>", ":NvimTreeToggle<CR>", { noremap = true, silent = true, desc = "Toggle NvimTree file explorer" })
map("n", "<A-`>", ":NvimTreeToggle<CR>", { noremap = true, silent = true, desc = "Toggle NvimTree file explorer" })

-- Buffer / Window navigation & Resizing
map("n", "<A-Left>", "<C-w>h", { noremap = true, silent = true, desc = "Move to left window" })
map("n", "<A-Right>", "<C-w>l", { noremap = true, silent = true, desc = "Move to right window" })

-- Resize window / buffer (Alt + Shift + Arrows)
map({ "n", "i", "v" }, "<A-S-Up>", "<cmd>resize +2<CR>", { noremap = true, silent = true, desc = "Increase window height" })
map({ "n", "i", "v" }, "<A-S-Down>", "<cmd>resize -2<CR>", { noremap = true, silent = true, desc = "Decrease window height" })
map({ "n", "i", "v" }, "<A-S-Left>", "<cmd>vertical resize -2<CR>", { noremap = true, silent = true, desc = "Decrease window width" })
map({ "n", "i", "v" }, "<A-S-Right>", "<cmd>vertical resize +2<CR>", { noremap = true, silent = true, desc = "Increase window width" })

map("n", ";", ":", { desc = "Enter command mode" })

-- Editing & Selection Shortcuts
map({ "n", "v" }, "`", "<esc><C-v>", { noremap = true, silent = true, desc = "Enter visual block mode" })
map({ "x" }, "<enter>", "<S-a>", { noremap = true, silent = true, desc = "Append text in visual block" })
map({ "i", "n" }, "<C-x>", '<esc>"+dd<esc>', { noremap = true, silent = true, desc = "Cut line to clipboard" })
map({ "v" }, "<C-x>", '"+dd<esc>', { noremap = true, silent = true, desc = "Cut line to clipboard" })

map({ "n" }, "<enter>", "i", { noremap = true, silent = true, desc = "Enter insert mode" })

map({ "n", "v" }, "<C-v>", '<esc>"+p', { noremap = true, silent = true, desc = "Paste from clipboard" })
map({ "i" }, "<C-v>", '<esc>"+p ==gi', { noremap = true, silent = true, desc = "Paste from clipboard in insert mode" })

map({ "v" }, "<C-c>", '"+y', { noremap = true, silent = true, desc = "Copy selection to clipboard" })
map({ "i", "n" }, "<C-c>", '<esc>"+yy', { noremap = true, silent = true, desc = "Copy line to clipboard" })
map({ "i", "n" }, "<C-d>", '<esc>"+yyp', { noremap = true, silent = true, desc = "Duplicate line" })

map({ "i", "n" }, "<C-Del>", "<esc>daw", { noremap = true, silent = true, desc = "Delete word forward" })
map({ "i", "n" }, "<C-Backspace>", "<C-w>", { noremap = true, silent = true, desc = "Delete word backward" })

map({ "i", "n", "v" }, "<C-z>", "<esc>u<esc>", { noremap = true, silent = true, desc = "Undo change" })

map("n", "<C-/>", "gcc", { desc = "Toggle comment on current line", remap = true })
map("i", "<C-/>", "<esc>gcci", { desc = "Toggle comment on current line", remap = true })
map("v", "<C-/>", "gc", { desc = "Toggle comment on selection", remap = true })

-- Folding
map("n", "<C-CR>", "za", { noremap = true, silent = true, desc = "Toggle fold under cursor" })
map("n", "<C-S-CR>", "zA", { noremap = true, silent = true, desc = "Toggle all folds recursively under cursor" })

-- Search text in buffer (Telescope)
map({ "i", "n" }, "<C-f>", "<cmd>Telescope current_buffer_fuzzy_find<CR>", { desc = "Search text in file (Telescope)" })

-- Quit & Save with format
map({ "i", "n", "v" }, "<C-q>", "<esc>:q<CR>", { noremap = true, silent = true, desc = "Quit current buffer" })
map({ "i", "n", "v" }, "<C-S-q>", "<esc>:qa<CR>", { noremap = true, silent = true, desc = "Quit Neovim" })
map({ "i", "n", "v" }, "<C-s>", function()
	local buftype = vim.bo.buftype
	if buftype == "nofile" or buftype == "prompt" or buftype == "terminal" then
		return
	end
	local filename = vim.fn.expand("%:t")
	if filename == "" then filename = "File" end
	pcall(vim.lsp.buf.format)
	local ok, err = pcall(vim.cmd, "w!")
	if ok then
		vim.notify(filename .. " saved", "info", { title = "Saving..." })
	else
		vim.notify("Could not save: " .. tostring(err), "error", { title = "Save Error" })
	end
end, { noremap = true, silent = true, desc = "Format & Save file" })

-- Move lines
map("n", "<A-Up>", ":m .-2<CR>==", { noremap = true, silent = true, desc = "Move line up" })
map("i", "<A-Up>", "<Esc>:m .-2<CR>==gi", { noremap = true, silent = true, desc = "Move line up" })
map("n", "<A-Down>", ":m .+1<CR>==", { noremap = true, silent = true, desc = "Move line down" })
map("i", "<A-Down>", "<Esc>:m .+1<CR>==gi", { noremap = true, silent = true, desc = "Move line down" })

-- Indenting
map({ "n", "v" }, "<Tab>", ">gv", { noremap = true, silent = true, desc = "Indent selection right" })
map({ "n", "v" }, "<S-Tab>", "<gv", { noremap = true, silent = true, desc = "Indent selection left" })

-- Fast Navigation (Ctrl + Arrows)
map({ "n", "v" }, "<C-Up>", "3k", { noremap = true, silent = true, desc = "Move 3 lines up" })
map({ "i" }, "<C-Up>", "<C-o>3k", { noremap = true, silent = true, desc = "Move 3 lines up" })
map({ "n", "v" }, "<C-Down>", "3j", { noremap = true, silent = true, desc = "Move 3 lines down" })
map({ "i" }, "<C-Down>", "<C-o>3j", { noremap = true, silent = true, desc = "Move 3 lines down" })
map({ "n", "v" }, "<C-Left>", "b", { noremap = true, silent = true, desc = "Move word left" })
map({ "i" }, "<C-Left>", "<C-o>b", { noremap = true, silent = true, desc = "Move word left" })
map({ "n", "v" }, "<C-Right>", "w", { noremap = true, silent = true, desc = "Move word right" })
map({ "i" }, "<C-Right>", "<C-o>w", { noremap = true, silent = true, desc = "Move word right" })

-- Line & Block selection
map({ "i", "n" }, "<S-Left>", "<esc>v<Left>", { noremap = true, silent = true, desc = "Select character left" })
map({ "i", "n" }, "<S-Right>", "<esc>v<Right>", { noremap = true, silent = true, desc = "Select character right" })
map({ "i", "n" }, "<C-S-Left>", "<esc>vib", { noremap = true, silent = true, desc = "Select inside block/parentheses" })
map({ "i", "n" }, "<C-S-Right>", "<esc>viw", { noremap = true, silent = true, desc = "Select inside word" })

map({ "i", "n" }, "<S-Down>", "<esc>^V", { noremap = true, silent = true, desc = "Select linewise down" })
map("v", "<S-Down>", ":<C-u>.+1<CR>gv", { noremap = true, silent = true, desc = "Extend selection down" })
map({ "i", "n" }, "<S-Up>", "<esc>$V", { noremap = true, silent = true, desc = "Select linewise up" })
map("v", "<S-Up>", ":<C-u>.-1<CR>gv", { noremap = true, silent = true, desc = "Extend selection up" })

-- Terminal Mode
--[[ Single <Esc> is sent to the running program (freebuff uses it to stop code).
     Double <Esc> exits terminal mode back to Normal so you can switch tabs / navigate. ]]
map("t", "<Esc><Esc>", "<C-\\><C-n>", { desc = "Exit terminal mode (Normal)" })
