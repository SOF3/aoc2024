def identify_number:
	[
		(.[:3] | tonumber? | tostring),
		(.[:2] | tonumber? | tostring),
		(.[:1] | tonumber? | tostring)
	] |
	.[0]
;

def parse_after_mul(continue_fn):
	identify_number as $left_number_str |
	if $left_number_str == null then continue_fn else
		.[($left_number_str | length):] |
		if .[0:1] != "," then continue_fn else
			.[1:] |
			identify_number as $right_number_str |
			if $right_number_str == null then continue_fn else
				.[($right_number_str | length):] |
				if .[0:1] != ")" then continue_fn else
					.[1:] |
					($left_number_str | tonumber) * ($right_number_str | tonumber) + continue_fn
				end
			end
		end
	end
;

# Finds the sum of mul()s in the receiver buffer.
# On failure, discard the substring processed in the current function and restart with the remaining data.
# If a mul() expression is correctly identified, continue with the remaining data and add the result to the current expression.
def d3q1:
	index("mul(") as $index |
	if $index == null then 0 else
		.[$index + 4:] | parse_after_mul(d3q1)
	end
;

# jq doesn't support calling functions defined after, but fortunately we can just pass the function:6
def d3q2_dont(d3q2_do):
	index("do()") as $index |
	if $index == null then 0 else
		.[$index+4:] |
		d3q2_do
	end
;

def d3q2_mul(d3q2_do):
	parse_after_mul(d3q2_do)
;

def d3q2_do:
	index("don't()") as $dont_index |
	index("mul(") as $mul_index |
	if $dont_index != null and $mul_index != null then
		if $dont_index < $mul_index then
			.[$dont_index+7:] | d3q2_dont(d3q2_do)
		else
			.[$mul_index+4:] | d3q2_mul(d3q2_do)
		end
	elif $dont_index != null then
		.[$dont_index+7:] | d3q2_dont(d3q2_do)
	elif $mul_index != null then
		.[$mul_index+4:] | d3q2_mul(d3q2_do)
	else 0 end
;

def d3q2: d3q2_do;
