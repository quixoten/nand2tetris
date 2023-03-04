#!/usr/bin/bash

set -o errexit
set -o errtrace
set -o functrace
set -o nounset

root_dir="$(readlink -f "$(dirname "$(readlink -f "$0")")/..")"
projects_dir="${root_dir}/projects"

reset-hdl-files() {
	while read -rs hdl; do
		(
			cd "$root_dir"
			git checkout 6195e199ead10d7d397117031283ef259233a35f -- "projects/${hdl##*projects/}"
		)
	done < <(find "${projects_dir}" -name "*.hdl")
}

reset-asm-files() {
	while read -rs asm; do
		(
			cd "$root_dir"
			git checkout 6195e199ead10d7d397117031283ef259233a35f -- "projects/${asm##*projects/}"
		)
	done < <(find "${projects_dir}/04" -name "*.asm")
}

restore-staged-files() {
	(
		cd "$root_dir"
		git restore --staged \*
	)
}

clean-ignored-files() {
	(
		cd "$root_dir"
		git clean -Xf
	)
}

reset-hdl-files
reset-asm-files
restore-staged-files
clean-ignored-files
