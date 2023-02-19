#!/usr/bin/bash

# $Id: CPUEmulator.sh,v 1.1 2014/06/17 21:14:01 marka Exp $
# mark.armbrust@pobox.com

tools_dir="$(readlink -f "$(dirname "$(readlink -f "$0")")")"
java_cmd="java -classpath "${CLASSPATH}:bin/classes:bin/lib/Hack.jar:bin/lib/HackGUI.jar:bin/lib/Simulators.jar:bin/lib/SimulatorsGUI.jar:bin/lib/Compilers.jar" CPUEmulatorMain"

if [[ "$#" -gt 1 || "$1" = "-h" || "$1" = "--help" ]]; then
	echo "Usage:"
	echo "    CPUEmulator.sh             Starts the CPU Emulator in interactive mode."
	echo "    CPUEmulator.sh FILE.tst    Starts the CPU Emulator and runs the File.tst"
	echo "                               test script.  The success/failure message"
	echo "                               is printed to the command console."
	echo "\$@ = $@"
	exit 0
fi

if [[ "$#" -eq 0 ]]; then
	(cd "${tools_dir}"; unset CDPATH; ${java_cmd} &)
else
	test="$(readlink -f "$1")"
	(cd "${tools_dir}"; unset CDPATH; ${java_cmd} "$test")
fi
