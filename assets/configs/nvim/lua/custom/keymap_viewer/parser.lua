local M = {}

local MODE_LABELS = {
	{ "n", "Normal" },
	{ "v", "Visual" },
	{ "i", "Insert" },
	{ "x", "VisualBlock" },
	{ "t", "Terminal" },
}

-- Lua patterns have no | alternation, so we use two patterns (same 3 captures:
-- mode, lhs, desc) — one for brace-list modes, one for quoted-string modes.
local MAP_BRACE = [=[map%(%s*(%b{})%s*,%s*["']([^"']+)["']%s*,.-desc%s*=%s*["']([^"']+)["']]=]
local MAP_QUOTED = [=[map%(%s*["']([^"']+)["']%s*,%s*["']([^"']+)["']%s*,.-desc%s*=%s*["']([^"']+)["']]=]

function M.parse_keymaps()
	local keymaps_list = {}
	local seen = {}

	local config_path = vim.fn.stdpath("config") .. "/lua/config/keymaps.lua"
	local f = io.open(config_path, "r")

	if f then
		local current_category = "General Shortcuts"
		for line in f:lines() do
			local cat_match = line:match("^%s*%-%-%s*(.+)%s*$")
			if cat_match and not cat_match:find("^%[%[") and not cat_match:find("^%-%-") then
				current_category = cat_match
			end

			local m1, l1, d1 = line:match(MAP_BRACE)
			local m2, l2, d2 = line:match(MAP_QUOTED)
			local mode, lhs, desc = m1 or m2, l1 or l2, d1 or d2
			if mode and lhs and desc then
				local modes = {}
				for _, def in ipairs(MODE_LABELS) do
					if mode:find(def[1]) then
						table.insert(modes, def[2])
					end
				end
				if #modes == 0 then modes = { "Normal" } end

				local mode_display = table.concat(modes, " / ")
				local key_id = lhs .. mode_display .. desc

				if not seen[key_id] then
					seen[key_id] = true
					table.insert(keymaps_list, {
						lhs = lhs,
						mode = mode_display,
						desc = desc,
						category = current_category,
					})
				end
			end
		end
		f:close()
	end

	-- Fallback to Neovim API if file parsing returned nothing
	if #keymaps_list == 0 then
		local modes = { "n", "v", "i", "x", "t" }
		for _, m in ipairs(modes) do
			local maps = vim.api.nvim_get_keymap(m)
			for _, map in ipairs(maps) do
				if map.desc and map.desc ~= "" and map.lhs and map.lhs ~= "" then
					table.insert(keymaps_list, {
						lhs = map.lhs,
						mode = m,
						desc = map.desc,
						category = "Neovim Keymaps",
					})
				end
			end
		end
	end

	return keymaps_list
end

return M
