local M = {}

local OLD_MARKER = "<<<<<<< OLD CODE (Git HEAD)"
local DIV_MARKER = "======="
local NEW_MARKER = ">>>>>>> NEW CODE (Current)"

function M.all_resolved(hunks, decisions)
	if #hunks == 0 then return false end
	for i in ipairs(hunks) do
		if not decisions[i] then return false end
	end
	return true
end

function M.render_buffer_lines(parts, decisions)
	local lines = {}
	local hunk_line_map = {} -- hunk_idx -> { start_line, end_line, is_resolved, choice }

	for _, part in ipairs(parts) do
		if part.type == "text" then
			for _, l in ipairs(part.lines) do
				table.insert(lines, l)
			end
		elseif part.type == "hunk" then
			local idx = part.hunk_idx
			local d = decisions[idx]
			local start_l = #lines + 1

			if d == "old" then
				for _, l in ipairs(part.old_lines) do
					table.insert(lines, l)
				end
				local end_l = math.max(start_l, #lines)
				hunk_line_map[idx] = { start_line = start_l, end_line = end_l, is_resolved = true, choice = "old" }
			elseif d == "new" then
				for _, l in ipairs(part.new_lines) do
					table.insert(lines, l)
				end
				local end_l = math.max(start_l, #lines)
				hunk_line_map[idx] = { start_line = start_l, end_line = end_l, is_resolved = true, choice = "new" }
			else
				table.insert(lines, OLD_MARKER)
				local old_hdr = #lines

				for _, l in ipairs(part.old_lines) do
					table.insert(lines, l)
				end

				table.insert(lines, DIV_MARKER)
				local div_hdr = #lines

				for _, l in ipairs(part.new_lines) do
					table.insert(lines, l)
				end

				table.insert(lines, NEW_MARKER)
				local new_hdr = #lines

				hunk_line_map[idx] = {
					start_line = start_l,
					end_line = new_hdr,
					old_hdr = old_hdr,
					div_hdr = div_hdr,
					new_hdr = new_hdr,
					is_resolved = false,
				}
			end
		end
	end

	return lines, hunk_line_map
end

function M.rebuild_final_content(parts, decisions)
	local lines = {}
	for _, part in ipairs(parts) do
		if part.type == "text" then
			for _, l in ipairs(part.lines) do
				table.insert(lines, l)
			end
		elseif part.type == "hunk" then
			local d = decisions[part.hunk_idx]
			if d == "old" then
				for _, l in ipairs(part.old_lines) do
					table.insert(lines, l)
				end
			elseif d == "new" then
				for _, l in ipairs(part.new_lines) do
					table.insert(lines, l)
				end
			end
		end
	end
	return lines
end

return M
