local M = {}

function M.hunk_at_line(hunks, line)
	for i = #hunks, 1, -1 do
		if line >= hunks[i].old then return i end
	end
	return 1
end

function M.find_next_undecided(hunks, decisions, cur_hunk)
	if #hunks == 0 then return cur_hunk end
	local next_idx = cur_hunk
	repeat
		next_idx = next_idx + 1
		if next_idx > #hunks then next_idx = 1 end
	until not decisions[next_idx] or next_idx == cur_hunk
	return next_idx
end

function M.find_prev_undecided(hunks, decisions, cur_hunk)
	if #hunks == 0 then return cur_hunk end
	local prev_idx = cur_hunk
	repeat
		prev_idx = prev_idx - 1
		if prev_idx < 1 then prev_idx = #hunks end
	until not decisions[prev_idx] or prev_idx == cur_hunk
	return prev_idx
end

return M
