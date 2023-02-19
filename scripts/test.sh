#!/usr/bin/bash

root_dir="$(readlink -f "$(dirname "$(readlink -f "$0")")/..")"
projects_dir="${root_dir}/projects"
tools_dir="${root_dir}/tools"
hardware_simulator="${tools_dir}/HardwareSimulator.sh"

while read -rs hdl; do
	if bash "$hardware_simulator" "${hdl%%.hdl}.tst" >/dev/null 2>&1; then
		echo -e "\e[32m✓ projects/${hdl##*projects/}\e[0m"
	else
		echo -e "\e[31m✗ projects/${hdl##*projects/}\e[0m"
	fi
done < <(find "${projects_dir}/01" -name "*.hdl")
