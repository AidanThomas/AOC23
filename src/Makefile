day ?= 1

new:
	@echo "use crate::parser; \n" \
		"\n" \
		"pub fn answer() {\n" \
		"match part1(false) {\n" \
		"Ok(_) => (),\n" \
		"Err(e) => panic!(\"{e}\"),\n" \
		"};\n" \
		"match part2(false) {\n" \
		"Ok(_) => (),\n" \
		"Err(e) => panic!(\"{e}\"),\n" \
		"};\n" \
		"}\n" \
		"\n" \
		"fn part1(real: bool) -> Result<(), String> {\n" \
		"let input: Vec<String>;\n" \
		"if real {\n" \
		"input = parser::read_by_line(\"inputs/day${day}/real.txt\")?;\n" \
		"} else {\n" \
		"input = parser::read_by_line(\"inputs/day${day}/test.txt\")?;\n" \
		"}\n" \
		"Ok(())\n" \
		"}\n" \
		"\n" \
		"fn part2(real: bool) -> Result<(), String> {\n" \
		"let input: Vec<String>;\n" \
		"if real {\n" \
		"input = parser::read_by_line(\"inputs/day${day}/real.txt\")?;\n" \
		"} else {\n" \
		"input = parser::read_by_line(\"inputs/day${day}/test.txt\")?;\n" \
		"}\n" \
		"Ok(())\n" \
		"}\n" \
		> day${day}.rs
	@mkdir inputs/day${day}
	@touch inputs/day${day}/real.txt
	@touch inputs/day${day}/test.txt
.PHONY: new
