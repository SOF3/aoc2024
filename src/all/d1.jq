def d1q1:
	split("\n") |
	map(
		split(" ") |
		map(tonumber?) |
		select(length == 2) |
		.[0] - .[1] | abs
	) |
	add
;

def uniq_count:
	group_by(.) |
	map(
		(.[0] | tostring) as $key |
		{$key: length}
	) |
	add
;

def d1q2_hash:
	split("\n") |
	map(
		split(" ") |
		map(tonumber?) |
		select(length == 2)
	) |
	(map(.[0]) | uniq_count) as $left |
	(map(.[1]) | uniq_count) as $right |
	$left | with_entries(.value *= ($right[.key | tostring] // 0) * (.key | tonumber)) |
	values | add
;
