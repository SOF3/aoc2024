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
