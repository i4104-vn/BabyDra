local M = {}

local function get_git_content(filepath)
	local rel_path = vim.fn.fnamemodify(filepath, ":.")
	local cmd = string.format("git show HEAD:%s 2>/dev/null", vim.fn.shellescape(rel_path))
	local out = vim.fn.systemlist(cmd)
	if vim.v.shell_error ~= 0 then return {} end
	return out
end

local function parse_hunk_header(header)
	local old_start, old_count, new_start, new_count = header:match("@@ %-(%d+),?(%d*) %+(%d+),?(%d*) @@")
	if not old_start then return nil end
	return {
		old_start = tonumber(old_start),
		old_count = tonumber(old_count) or 1,
		new_start = tonumber(new_start),
		new_count = tonumber(new_count) or 1,
	}
end

function M.build_with_hunks(filepath)
	if not filepath or vim.fn.filereadable(filepath) == 0 then return nil end

	local disk_lines = vim.fn.readfile(filepath)
	local orig_lines = get_git_content(filepath)

	-- If it's a newly created file (not present in git HEAD)
	if #orig_lines == 0 then
		local parts = {
			{
				type = "hunk",
				hunk_idx = 1,
				old_lines = {},
				new_lines = disk_lines,
				orig_line = 1,
			}
		}
		local hunks = { parts[1] }
		return { parts = parts, hunks = hunks }
	end

	local rel_path = vim.fn.fnamemodify(filepath, ":.")
	local diff_cmd = string.format("git diff -U3 HEAD -- %s", vim.fn.shellescape(rel_path))
	local diff_output = vim.fn.systemlist(diff_cmd)

	local parts = {}
	local hunks = {}
	local orig_idx = 1
	local disk_idx = 1

	local current_text_lines = {}

	local function flush_text()
		if #current_text_lines > 0 then
			table.insert(parts, { type = "text", lines = current_text_lines })
			current_text_lines = {}
		end
	end

	local i = 1
	while i <= #diff_output do
		local line = diff_output[i]
		if line:sub(1, 2) == "@@" then
			local h = parse_hunk_header(line)
			if h then
				while orig_idx < h.old_start and disk_idx < h.new_start do
					table.insert(current_text_lines, disk_lines[disk_idx] or orig_lines[orig_idx])
					orig_idx = orig_idx + 1
					disk_idx = disk_idx + 1
				end

				local hunk_old_lines = {}
				local hunk_new_lines = {}
				i = i + 1

				while i <= #diff_output and not diff_output[i]:sub(1, 2):find("@@") do
					local dline = diff_output[i]
					local prefix = dline:sub(1, 1)
					local content = dline:sub(2)

					if prefix == "-" then
						table.insert(hunk_old_lines, content)
						orig_idx = orig_idx + 1
					elseif prefix == "+" then
						table.insert(hunk_new_lines, content)
						disk_idx = disk_idx + 1
					elseif prefix == " " then
						if #hunk_old_lines > 0 or #hunk_new_lines > 0 then
							flush_text()
							local hunk_obj = {
								type = "hunk",
								hunk_idx = #hunks + 1,
								old_lines = hunk_old_lines,
								new_lines = hunk_new_lines,
								orig_line = h.new_start,
							}
							table.insert(parts, hunk_obj)
							table.insert(hunks, hunk_obj)
							hunk_old_lines = {}
							hunk_new_lines = {}
						end
						table.insert(current_text_lines, content)
						orig_idx = orig_idx + 1
						disk_idx = disk_idx + 1
					end
					i = i + 1
				end

				if #hunk_old_lines > 0 or #hunk_new_lines > 0 then
					flush_text()
					local hunk_obj = {
						type = "hunk",
						hunk_idx = #hunks + 1,
						old_lines = hunk_old_lines,
						new_lines = hunk_new_lines,
						orig_line = h.new_start,
					}
					table.insert(parts, hunk_obj)
					table.insert(hunks, hunk_obj)
				end
				i = i - 1
			end
		end
		i = i + 1
	end

	while disk_idx <= #disk_lines do
		table.insert(current_text_lines, disk_lines[disk_idx])
		disk_idx = disk_idx + 1
	end
	flush_text()

	return { parts = parts, hunks = hunks }
end

return M
