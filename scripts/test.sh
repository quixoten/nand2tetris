#!/usr/bin/bash

root_dir="$(readlink -f "$(dirname "$(readlink -f "$0")")/..")"
projects_dir="${root_dir}/projects"
tools_dir="${root_dir}/tools"
hardware_simulator="${tools_dir}/HardwareSimulator.sh"
cpu_emulator="${tools_dir}/CPUEmulator.sh"


for project_number in 01 02 03 04; do
	while read -rs tst; do
		if [[ $project_number -eq 4 ]]; then
			test_tool="${cpu_emulator}"
		else
			test_tool="${hardware_simulator}"
		fi

		test_output=$(bash "$test_tool" "${tst}" 2>&1)

		if [[ $? -eq 0 ]]; then
			echo -e "\e[32m✓ projects/${tst##*projects/}\e[0m"
		else
			printf "\e[31m%-30s\e[0m" "✗ projects/${tst##*projects/}"
			echo "${test_output}" | sed 's/^/    /'
		fi
	done < <(find "${projects_dir}/${project_number}" -name "*.tst")
done
