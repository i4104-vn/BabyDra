local M = {}

local state_file = vim.fn.stdpath("cache") .. "/sidebar_resolved_changes.json"

local function load_state()
	if vim.fn.filereadable(state_file) == 1 then
		local lines = vim.fn.readfile(state_file)
		local raw = table.concat(lines, "\n")
		local ok, data = pcall(vim.json.decode, raw)
		if ok and type(data) == "table" then return data end
	end
	return {}
end

local function save_state(state)
	local ok, json = pcall(vim.json.encode, state)
	if ok and json then vim.fn.writefile({ json }, state_file) end
end

local function file_sha256(filepath)
	if vim.fn.filereadable(filepath) == 1 then
		return vim.fn.sha256(table.concat(vim.fn.readfile(filepath), "\n"))
	end
	return ""
end

function M.is_resolved(filepath)
	local st = load_state()
	local entry = st[filepath]
	if not entry then return false end
	return entry.hash == file_sha256(filepath)
end

function M.mark_resolved(filepath)
	local st = load_state()
	st[filepath] = { hash = file_sha256(filepath), timestamp = os.time() }
	save_state(st)
end

function M.unmark_resolved(filepath)
	local st = load_state()
	st[filepath] = nil
	save_state(st)
end

return M
