#!/usr/bin/bash

root_dir="$(readlink -f "$(dirname "$(readlink -f "$0")")/..")"
projects_dir="${root_dir}/projects"
tools_dir="${root_dir}/tools"
hardware_simulator="${tools_dir}/HardwareSimulator.sh"

for project_number in 02; do
	while read -rs hdl; do
		test_output=$(bash "$hardware_simulator" "${hdl%%.hdl}.tst" 2>&1)

		if [[ $? -eq 0 ]]; then
			echo -e "\e[32m✓ projects/${hdl##*projects/}\e[0m"
		else
			printf "\e[31m%-30s\e[0m" "✗ projects/${hdl##*projects/}"
			echo "${test_output}" | sed 's/^/    /'
		fi
	done < <(find "${projects_dir}/${project_number}" -name "*.hdl")
done
