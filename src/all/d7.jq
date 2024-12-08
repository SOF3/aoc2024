def parse_line:
	split(": ") |
	select(length == 2) |
	{
		result: .[0] | tonumber,
		operands: (.[1] | split(" ") | map(tonumber)),
	}
;

# equivalent to haskell functions of the same name
def list_init: .[:length - 1];
def list_last: .[length - 1];

def is_valid_by_add(is_valid):
	(.result - (.operands | list_last)) as $sub |
	if $sub >= 0 then
		{
			result: $sub,
			operands: (.operands | list_init),
		} | is_valid
	else
		false
	end
;

def is_valid_by_mul(is_valid):
	if .result % (.operands | list_last) == 0 then
		{
			result: (.result / (.operands | list_last)),
			operands: (.operands | list_init),
		} | is_valid
	else
		false
	end
;

def is_valid_q1:
	if .operands | length == 0 then
		.result == 0
	else
		is_valid_by_add(is_valid_q1) or is_valid_by_mul(is_valid_q1)
	end
;

def d7q1:
	split("\n") | map(
		parse_line |
		select(is_valid_q1) |
		.result
	) |
	add
;

def is_valid_by_concat(is_valid):
	(.result | tostring) as $result_string |
	(.operands | list_last) as $last |
	($last | tostring) as $last_string |
	if $result_string | endswith($last_string) then
		{
			result: (
				$result_string |
				.[:length - ($last_string | length)] |
				tonumber
			),
			operands: (.operands | list_init),
		} | is_valid
	else
		false
	end
;

def is_valid_q2:
	if .operands | length == 0 then
		.result == 0
	else
		is_valid_by_add(is_valid_q2) or is_valid_by_mul(is_valid_q2) or is_valid_by_concat(is_valid_q2)
	end
;

def d7q2:
	split("\n") | map(
		parse_line |
		select(is_valid_q2) |
		.result
	) |
	add
;
