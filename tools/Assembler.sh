#!/usr/bin/env sh

# $Id: Assembler.sh,v 1.1 2014/06/17 21:14:01 marka Exp $
# mark.armbrust@pobox.com

tools_dir="$(readlink -f "$(dirname "$(readlink -f "$0")")")"
java_cmd="java -classpath ${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Compilers.jar:bin/lib/AssemblerGUI.jar:bin/lib/TranslatorsGUI.jar HackAssemblerMain"

if [[ "$#" -gt 1 || "$1" = "-h" || "$1" = "--help" ]]; then
	echo "Usage:"
	echo "    Assembler.sh             Starts the assembler in interactive mode."
	echo "    Assembler.sh FILE[.asm]  Assembles FILE.asm to FILE.hack."
	exit 0
fi

if [[ "$#" -eq 0 ]]; then
	(cd "${tools_dir}"; unset CDPATH; ${java_cmd} &)
else
	asm="$(readlink -f "$1")"
	echo "Assembling $asm"
	(cd "${tools_dir}"; ${java_cmd} "$asm")
fi
