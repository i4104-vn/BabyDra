-- Topic: Review Controller Facade
-- Orchestrates diff review windows using topic submodules

local M = {}

local diff_builder = require("custom.sidebar.tabs.changes.git.diff")
local resolver = require("custom.sidebar.tabs.changes.fs.resolver")
local tmp = require("custom.sidebar.tabs.changes.fs.tmp")

local highlights = require("custom.sidebar.tabs.changes.ui.highlights")
local hunk_list = require("custom.sidebar.tabs.changes.ui.hunks")
local shortcuts_panel = require("custom.sidebar.tabs.changes.ui.shortcuts")
local layout = require("custom.sidebar.tabs.changes.ui.layout")

local keymaps = require("custom.sidebar.tabs.changes.review.keymaps")
local engine = require("custom.sidebar.tabs.changes.review.engine")
local navigation = require("custom.sidebar.tabs.changes.review.navigation")

-- State variables
M.filepath = nil
M.tmp_filepath = nil
M.buf, M.sidebar_buf, M.shortcut_buf = nil, nil, nil
M.main_win, M.sidebar_win, M.shortcut_win = nil, nil, nil

M.parts, M.hunks, M.decisions = {}, {}, {}
M.cur_hunk = 1
M.hunk_line_map = {}
M.hunk_to_sidebar_line, M.sidebar_line_to_hunk = {}, {}

-- Partial decisions cache logic (I/O optimization)
local partial_file = vim.fn.stdpath("cache") .. "/sidebar_partial_decisions.json"
local cached_partials = nil

local function load_all_partials()
	if cached_partials then return cached_partials end
	if vim.fn.filereadable(partial_file) == 1 then
		local lines = vim.fn.readfile(partial_file)
		local ok, data = pcall(vim.json.decode, table.concat(lines, "\n"))
		if ok and type(data) == "table" then
			cached_partials = data
			return cached_partials
		end
	end
	cached_partials = {}
	return cached_partials
end

local function save_all_partials()
	if not cached_partials then return end
	local ok, json = pcall(vim.json.encode, cached_partials)
	if ok and json then vim.fn.writefile({ json }, partial_file) end
end

local function get_partial_decisions(filepath)
	return load_all_partials()[filepath] or {}
end

local function set_partial_decisions(filepath, decisions)
	load_all_partials()[filepath] = decisions
	save_all_partials()
end

local function clear_partial_decisions(filepath)
	load_all_partials()[filepath] = nil
	save_all_partials()
end

-- Core API
function M.is_open()
	return M.main_win and vim.api.nvim_win_is_valid(M.main_win)
end

local function refresh_main_buffer()
	if not M.buf or not vim.api.nvim_buf_is_valid(M.buf) then return end
	local lines, line_map = engine.render_buffer_lines(M.parts, M.decisions)
	M.hunk_line_map = line_map

	vim.bo[M.buf].modifiable = true
	vim.api.nvim_buf_set_lines(M.buf, 0, -1, false, lines)
	vim.bo[M.buf].modifiable = false
	highlights.apply_main_hl(M.buf, M.hunk_line_map, M.cur_hunk)
end

local function render_sidebar()
	M.hunk_to_sidebar_line, M.sidebar_line_to_hunk = hunk_list.render(M.sidebar_buf, M.sidebar_win, M.hunks, M.decisions, M.cur_hunk)
end

function M.jump_to_hunk(idx)
	if idx < 1 or idx > #M.hunks then
		vim.notify(idx < 1 and "At first hunk" or "No more hunks", "warn")
		return
	end
	M.cur_hunk = idx
	refresh_main_buffer()
	render_sidebar()

	if M.is_open() and M.hunk_line_map[idx] then
		pcall(vim.api.nvim_win_set_cursor, M.main_win, { M.hunk_line_map[idx].start_line, 0 })
	end
end

function M.jump_next() M.jump_to_hunk(M.cur_hunk + 1) end
function M.jump_prev() M.jump_to_hunk(M.cur_hunk - 1) end
function M.jump_first() M.jump_to_hunk(1) end
function M.jump_last() M.jump_to_hunk(#M.hunks) end
function M.jump_next_undecided() M.jump_to_hunk(navigation.find_next_undecided(M.hunks, M.decisions, M.cur_hunk)) end
function M.jump_prev_undecided() M.jump_to_hunk(navigation.find_prev_undecided(M.hunks, M.decisions, M.cur_hunk)) end

local function sync_from_left_cursor()
	if not M.is_open() or vim.api.nvim_get_current_win() ~= M.main_win then return end
	local line = vim.api.nvim_win_get_cursor(M.main_win)[1]
	for idx, info in pairs(M.hunk_line_map) do
		if line >= info.start_line and line <= info.end_line and idx ~= M.cur_hunk then
			M.cur_hunk = idx
			highlights.apply_main_hl(M.buf, M.hunk_line_map, M.cur_hunk)
			render_sidebar()
			return
		end
	end
end

local function sync_from_right_cursor()
	if not M.sidebar_win or not vim.api.nvim_win_is_valid(M.sidebar_win) or vim.api.nvim_get_current_win() ~= M.sidebar_win then return end
	local idx = M.sidebar_line_to_hunk[vim.api.nvim_win_get_cursor(M.sidebar_win)[1]]
	if idx and idx ~= M.cur_hunk then M.jump_to_hunk(idx) end
end

function M.close_and_focus()
	M.close()
	require("custom.sidebar.tabs.changes").focus_changes()
end

local function finish_and_save()
	local content = engine.rebuild_final_content(M.parts, M.decisions)
	vim.notify(("All %d hunks decided — saving %s"):format(#M.hunks, vim.fn.fnamemodify(M.filepath, ":t")), "info")
	clear_partial_decisions(M.filepath)
	resolver.finalize_per_hunk(M.filepath, content)
	if M.tmp_filepath then
		tmp.cleanup_tmp_file(M.tmp_filepath)
		M.tmp_filepath = nil
	end
	tmp.clear_project_tmp()
end

local function process_decision_action()
	if M.filepath then set_partial_decisions(M.filepath, M.decisions) end
	refresh_main_buffer()
	render_sidebar()
	if engine.all_resolved(M.hunks, M.decisions) then
		finish_and_save()
		return true
	end
	return false
end

function M.decide_current(side)
	if M.cur_hunk < 1 or M.cur_hunk > #M.hunks then return vim.notify("No hunk selected", "warn") end
	M.decisions[M.cur_hunk] = side
	if not process_decision_action() and side ~= nil then M.jump_next_undecided() end
end

function M.decide_all(side)
	for i in ipairs(M.hunks) do M.decisions[i] = side end
	process_decision_action()
end

local function setup_buffer_syntax(buf, filepath, lines)
	local ft = vim.filetype.match({ filename = filepath }) or vim.filetype.match({ contents = lines })
	if not ft or ft == "" then
		local ext_map = { rs="rust", js="javascript", ts="typescript", jsx="javascriptreact", tsx="typescriptreact", py="python", sh="bash", css="css", json="json", toml="toml", yaml="yaml", md="markdown", lua="lua", c="c", cpp="cpp" }
		ft = ext_map[vim.fn.fnamemodify(filepath, ":e")] or vim.fn.fnamemodify(filepath, ":e")
	end
	if ft and ft ~= "" then
		local old_ei = vim.o.eventignore
		vim.o.eventignore = "all"
		pcall(function() vim.bo[buf].filetype = ft end)
		vim.o.eventignore = old_ei
		pcall(function() vim.bo[buf].syntax = ft end)
		pcall(function() vim.treesitter.start(buf, ft) end)
	end
end

function M.open(filepath)
	if not filepath or vim.fn.filereadable(filepath) == 0 then return end
	if M.is_open() then M.close() end

	local data = diff_builder.build_with_hunks(filepath)
	if not data or not data.hunks or #data.hunks == 0 then return vim.notify("No modified hunks found in " .. filepath, "info") end

	M.filepath = filepath
	M.parts, M.hunks = data.parts, data.hunks
	M.decisions = get_partial_decisions(filepath)
	M.cur_hunk = 1

	local initial_lines = engine.render_buffer_lines(M.parts, M.decisions)
	M.tmp_filepath = tmp.create_tmp_file(filepath, initial_lines)

	M.buf = vim.api.nvim_create_buf(true, false)
	pcall(vim.api.nvim_buf_set_name, M.buf, "FloatingDiff: " .. filepath)
	setup_buffer_syntax(M.buf, filepath, initial_lines)

	for _, k in ipairs({"buftype", "bufhidden", "buflisted", "swapfile", "modifiable"}) do
		vim.bo[M.buf][k] = (k == "buftype") and "nofile" or (k == "bufhidden") and "wipe" or false
	end

	M.sidebar_buf, M.shortcut_buf = vim.api.nvim_create_buf(false, true), vim.api.nvim_create_buf(false, true)
	M.main_win, M.sidebar_win, M.shortcut_win = layout.create_windows(filepath, M.buf, M.sidebar_buf, M.shortcut_buf, #M.hunks)

	refresh_main_buffer()
	render_sidebar()
	shortcuts_panel.render(M.shortcut_buf)

	keymaps.bind_all(M.buf, M.sidebar_buf, M.shortcut_buf, M.main_win, M.sidebar_win, {
		decide_current = M.decide_current, decide_all = M.decide_all,
		jump_next = M.jump_next, jump_prev = M.jump_prev, jump_first = M.jump_first, jump_last = M.jump_last,
		jump_next_undecided = M.jump_next_undecided, jump_prev_undecided = M.jump_prev_undecided, jump_to_hunk = M.jump_to_hunk,
		get_cur_hunk = function() return M.cur_hunk end, get_sidebar_line_to_hunk = function() return M.sidebar_line_to_hunk end,
		close_and_focus = M.close_and_focus,
	})

	vim.api.nvim_create_autocmd("CursorMoved", { buffer = M.buf, callback = sync_from_left_cursor })
	vim.api.nvim_create_autocmd("CursorMoved", { buffer = M.sidebar_buf, callback = sync_from_right_cursor })

	M.jump_to_hunk(1)
	vim.api.nvim_set_current_win(M.main_win)
end

function M.close()
	for _, win in ipairs({ M.shortcut_win, M.sidebar_win, M.main_win }) do
		if win and vim.api.nvim_win_is_valid(win) then pcall(vim.api.nvim_win_close, win, true) end
	end
	for _, buf in ipairs({ M.shortcut_buf, M.sidebar_buf, M.buf }) do
		if buf and vim.api.nvim_buf_is_valid(buf) then pcall(vim.api.nvim_buf_delete, buf, { force = true }) end
	end
	if engine.all_resolved(M.hunks, M.decisions) and M.tmp_filepath then
		tmp.cleanup_tmp_file(M.tmp_filepath)
		tmp.clear_project_tmp()
	end
	M.main_win, M.sidebar_win, M.shortcut_win, M.buf, M.sidebar_buf, M.shortcut_buf = nil, nil, nil, nil, nil, nil
	M.filepath, M.tmp_filepath, M.parts, M.hunks, M.decisions = nil, nil, {}, {}, {}
	M.cur_hunk = 1
	M.hunk_line_map, M.hunk_to_sidebar_line, M.sidebar_line_to_hunk = {}, {}, {}
end

return M
