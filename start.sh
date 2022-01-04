arg=$1

while true
do
	cargo run --release $arg
	printf "\x1b[1;32mWill restart in 5 seconds.\x1b[0m\n"
	printf "Exit status:\x1b[31m %d \x1b[0m\n" "$?"
	printf " -> to \x1b[31mexit\x1b[0m, press Ctrl-C again!\n"
	sleep 5
done
