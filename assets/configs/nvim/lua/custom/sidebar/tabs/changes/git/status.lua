local M = {}

function M.get_modified_files()
	local output = vim.fn.systemlist("git status --porcelain -uall")
	local files = {}

	for _, line in ipairs(output) do
		if #line >= 4 then
			local status = line:sub(1, 2)
			local file = line:sub(4)

			-- Handle renamed/moved file syntax: "R  old -> new"
			if file:find(" %-> ") then
				file = file:match("%-> (.+)$")
			end

			-- Strip trailing slash if present
			if file:sub(-1) == "/" then
				file = file:sub(1, -2)
			end

			-- Ensure entry is a valid file (not a directory) and not deleted/renamed/tmp
			local is_dir = vim.fn.isdirectory(file) == 1
			local is_deleted = status:find("D") ~= nil
			local is_renamed = status:find("R") ~= nil
			local is_tmp = file:find("%.tmp$") or file:find("nvim_tmp") or file:find("^%.tmp") or file:find("/%.tmp")

			if not is_dir and not is_deleted and not is_renamed and not is_tmp then
				local name = vim.fn.fnamemodify(file, ":t")
				if name and name ~= "" then
					local label = "[M]"
					if status:find("%?") or status:find("A") then
						label = "[+]"
					elseif status:find("U") then
						label = "[U]"
					end
					table.insert(files, { status = status, label = label, file = file })
				end
			end
		end
	end

	return files
end

return M
