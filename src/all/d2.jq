def windows($size):
	[
		range(length - 1) as $index |
		[.[$index], .[$index+1]]
	]
;

def classify_pair:
	.[0] - .[1] |
	if 1 <= . and . <= 3 then 1
	elif 1 <= -. and -. <= 3 then -1
	else 0 end
;

def all_equal:
	if length > 1 then
		.[0] as $first |
		all(. == $first)
	else true end
;

def is_list_safe:
	debug("is_list_safe", .) |
	windows(2) |
	map(classify_pair) |
	all_equal
;

def d2q1:
	split("\n") |
	map(
		split(" ") |
		map(tonumber?) |
		select(length > 0) |
		select(is_list_safe)
	) |
	length
;

def list_skip($index):
	debug("list_skip", $index) |
	.[:$index] + .[$index+1:]
;

# Incorrect solution, TODO fix
def d2q2:
	split("\n") |
	map(
		split(" ") |
		map(tonumber?) |
		select(length > 0) |
		select(is_list_safe or ([
			range(length) as $skip_index |
			list_skip($skip_index) |
			is_list_safe
		] | any(.)))
	) |
	length
;
