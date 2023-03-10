#!/usr/bin/bash

set -o errexit
set -o errtrace
set -o functrace
set -o nounset

root_dir="$(readlink -f "$(dirname "$(readlink -f "$0")")/..")"
projects_dir="${root_dir}/projects"
tools_dir="${root_dir}/tools"
hardware_simulator="${tools_dir}/HardwareSimulator.sh"
cpu_emulator="${tools_dir}/CPUEmulator.sh"


test-hdl-project() {
  declare project_number="$1"

  while read -rs tst; do
    if [[ $project_number -eq 4 ]]; then
      test_tool="${cpu_emulator}"
    else
      test_tool="${hardware_simulator}"
    fi

    if test_output="$(bash "$test_tool" "${tst}" 2>&1)"; then
      echo -e "\e[32m✓ projects/${tst##*projects/}\e[0m"
    else
      printf "\e[31m%-30s\e[0m" "✗ projects/${tst##*projects/}"
      echo "${test_output}" | sed 's/^/    /'
    fi
  done < <(find "${projects_dir}/${project_number}" -name "*.tst")
}


test-project-six() {
	while read -rs asm; do
		(
			cd "${root_dir}/assembler/rust"
			cargo run "${asm}" >/dev/null 2>&1
		)

		if diff_output=$(diff -u --color=always "${asm%.asm}.cmp" "${asm%.asm}.hack"); then
			echo -e "\e[32m✓ projects/${asm##*projects/}\e[0m"
		else
			printf "\e[31m%-30s\e[0m\n" "✗ projects/${asm##*projects/}"
			echo -e "${diff_output}" | sed 's/^/    /'
		fi
	done < <(find "${projects_dir}/06" -name "*.asm")
}


test-project-seven() {
	while read -rs vm; do
		(
			cd "${root_dir}/translator/rust"
			cargo run "${vm}" >/dev/null 2>&1
		)

		tst="${vm%.vm}.tst"
		
		if test_output=$(bash "$cpu_emulator" "${tst}" 2>&1); then
			echo -e "\e[32m✓ projects/${tst##*projects/}\e[0m"
		else
			printf "\e[31m%-30s\e[0m" "✗ projects/${tst##*projects/}"
			echo "${test_output}" | sed 's/^/    /'
		fi
	done < <(find "${projects_dir}/07" -name "*.vm")
}


test-project-eight() {
	while read -rs tst; do
		sourcedir="${tst%/*}"
    sourcefile="${sourcedir}/${sourcedir##*/}.vm"

    if [[ -e "${sourcefile}" ]]; then
      source="${sourcefile}"
    else
      source="${sourcedir}"
    fi

    (
      cd "${root_dir}/translator/rust"
      cargo run "${source}" >/dev/null 2>&1
    )

		if test_output=$(bash "$cpu_emulator" "${tst}" 2>&1); then
			echo -e "\e[32m✓ projects/${tst##*projects/}\e[0m"
		else
			printf "\e[31m%-30s\e[0m" "✗ projects/${tst##*projects/}"
			echo "${test_output}" | sed 's/^/    /'
		fi
	done < <(find "${projects_dir}/08" -name "*.tst" ! -name "*VME.tst")
}

test-hdl-project 01
test-hdl-project 02
test-hdl-project 03
test-hdl-project 04
test-hdl-project 05
test-project-six
test-project-seven
test-project-eight
