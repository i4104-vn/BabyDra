vim.opt.shiftwidth = 4
vim.opt.tabstop = 4

vim.opt.cursorline = true
vim.opt.termguicolors = true

vim.o.ignorecase = true
vim.o.smartcase = true
vim.o.number = true
vim.o.autoindent = true

vim.opt.fillchars = { eob = " " }

-- Font configuration (default VS Code font: Cascadia Code / Consolas)
vim.opt.guifont = "CaskaydiaCove Nerd Font:h12,Cascadia Code:h12,Consolas:h12"

-- Lower key timeout so a single <Esc> reaches the terminal program quickly
-- (freebuff uses <Esc> to stop code) while <Esc><Esc> still exits terminal mode.
vim.o.timeoutlen = 400
