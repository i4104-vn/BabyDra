local M = {}
local state = require("custom.sidebar.tabs.changes.git.state")

local undo_history = {}

local function reload_buffer_if_open(filepath)
	for _, buf in ipairs(vim.api.nvim_list_bufs()) do
		if vim.api.nvim_buf_is_valid(buf) and vim.api.nvim_buf_get_name(buf) == filepath then
			vim.api.nvim_buf_call(buf, function() vim.cmd("checktime") end)
		end
	end
end

local function is_created_file(filepath)
	local rel_path = vim.fn.fnamemodify(filepath, ":.")
	local cmd = string.format("git show HEAD:%s 2>/dev/null", vim.fn.shellescape(rel_path))
	local out = vim.fn.systemlist(cmd)
	return vim.v.shell_error ~= 0 or #out == 0
end

local function write_and_finalize(filepath, content, message, choice)
	if not filepath or filepath == "" then
		vim.notify("Invalid filepath for resolution", "error")
		return
	end

	-- If content is empty (e.g. user chose OLD on a created file), delete the created file
	if #content == 0 then
		if vim.fn.filereadable(filepath) == 1 then
			pcall(vim.fn.delete, filepath)
			vim.notify("Deleted created file: " .. vim.fn.fnamemodify(filepath, ":t"), "info")
		end
		state.unmark_resolved(filepath)
		reload_buffer_if_open(filepath)
		return
	end

	local created = is_created_file(filepath)
	local current_disk = nil
	if vim.fn.filereadable(filepath) == 1 then
		current_disk = vim.fn.readfile(filepath)
	end

	local ok, err = pcall(vim.fn.writefile, content, filepath)
	if not ok then
		vim.notify("Failed to write resolved file: " .. tostring(err), "error")
		return
	end

	table.insert(undo_history, {
		filepath = filepath,
		previous_content = current_disk,
		is_created = created,
		choice = choice,
		timestamp = os.time(),
	})

	state.mark_resolved(filepath)
	reload_buffer_if_open(filepath)
	vim.notify(message, "info")
end

function M.finalize_per_hunk(filepath, content)
	write_and_finalize(filepath, content, "All hunks resolved for " .. vim.fn.fnamemodify(filepath, ":t"), "per_hunk")
end

function M.undo_last_resolution()
	if #undo_history == 0 then
		vim.notify("No resolution to undo", "warn")
		return false
	end

	local last = table.remove(undo_history)
	local filename = vim.fn.fnamemodify(last.filepath, ":t")

	if last.is_created then
		-- File was newly created: Undo deletes it completely from disk!
		if vim.fn.filereadable(last.filepath) == 1 then
			pcall(vim.fn.delete, last.filepath)
		end
		vim.notify("Deleted created file: " .. filename, "info")
	else
		if last.previous_content then
			vim.fn.writefile(last.previous_content, last.filepath)
		end
		vim.notify("Undid resolution for " .. filename, "info")
	end

	state.unmark_resolved(last.filepath)
	reload_buffer_if_open(last.filepath)
	return true
end

return M
