local M = {}

function M.get_dir()
	local cwd = vim.fn.getcwd()
	local proj_name = vim.fn.fnamemodify(cwd, ":t")
	if not proj_name or proj_name == "" then proj_name = "default" end
	local dir = vim.fn.expand("~/.cache/nvim_tmp") .. "/" .. proj_name
	vim.fn.mkdir(dir, "p")
	return dir
end

function M.get_rel_path(filepath)
	local cwd = vim.fn.fnamemodify(vim.fn.getcwd(), ":p")
	local abs_path = vim.fn.fnamemodify(filepath, ":p")
	if abs_path:sub(1, #cwd) == cwd then
		return abs_path:sub(#cwd + 1)
	end
	return vim.fn.fnamemodify(filepath, ":t")
end

function M.create_tmp_file(filepath, content_lines)
	local proj_dir = M.get_dir()
	local rel_path = M.get_rel_path(filepath)

	local tmp_path = proj_dir .. "/" .. rel_path
	local tmp_parent = vim.fn.fnamemodify(tmp_path, ":h")
	vim.fn.mkdir(tmp_parent, "p")

	vim.fn.writefile(content_lines, tmp_path)
	return tmp_path
end

function M.cleanup_tmp_file(tmp_path)
	if tmp_path and vim.fn.filereadable(tmp_path) == 1 then
		pcall(vim.fn.delete, tmp_path)
	end
end

function M.clear_project_tmp()
	local proj_dir = M.get_dir()
	local files = vim.fn.glob(proj_dir .. "/**/*", false, true)
	for _, f in ipairs(files) do
		if vim.fn.isdirectory(f) == 0 then
			pcall(vim.fn.delete, f)
		end
	end
end

return M
