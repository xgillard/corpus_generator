#! /bin/bash
###############################################################################
# This script can be used to stop start or restart the corpus server.         #
#                                                                             #
# Usage:                                                                      #
#  server.sh start   # to start the server                                    #
#  server.sh stop    # to kill the server                                     #
#  server.sh restart # to stop and restart the server (eg to reload config)   #
#                                                                             #
# Author: X. Gillard                                                          #
# Date  : Nov. 3rd, 2021                                                      #
###############################################################################

export HERE="/root/bin.actix"
export ROCKET_PROFILE=release

# Just make sure the current working directory is ok
cd $HERE

# just start the server
function start_server () {
	nohup $HERE/corpus --endpoint="[::]:443" --cert="$HERE/private/linfo2263.pem" --key="$HERE/private/linfo2263.key" --corpus-dir="${HERE}/public/corpus/" >> "$HERE/server.log" &
}

# gets the pid of the service, then kill it
function stop_server () {
	# This is quite a bit of a complex command, so here is what it does:
	#
	# ps ax            -> list all the running processes
	# grep corpus      -> return only those entries having the word corpus in it
	# grep -v grep     -> but exclude the lines that also contain the word grep 
	#                     (this is useful to exclude the previous grep command 
    #                     from output
	# awk '{print $1}' -> print only the first column of the output 
	# xargs -L 1 -J %  -> run the kill command once for each line in stdin and 
	#                     use % as a placeholder for the pid in the kill command
	ps ax \
		| grep "corpus"    \
		| grep -v "grep"   \
		| awk '{print $1}' \
		| xargs -L 1 kill
}

case $1 in
	"start")
		now=$(date)
		echo "--------------------------------------------------------------------------------------" >> "$HERE/server.log"
		echo "--------------------- Starting $now  -------------------------"                         >> "$HERE/server.log"
		echo "--------------------------------------------------------------------------------------" >> "$HERE/server.log"
		start_server
		;;
	"stop")
		now=$(date)
		echo "--------------------------------------------------------------------------------------" >> "$HERE/server.log"
		echo "--------------------- Stopping $now --------------------------"                         >> "$HERE/server.log"
		echo "--------------------------------------------------------------------------------------" >> "$HERE/server.log"
		stop_server
		;;
	"restart"):
		now=$(date)
		echo "--------------------------------------------------------------------------------------" >> "$HERE/server.log"
		echo "--------------------- Restarting $now ------------------------"                         >> "$HERE/server.log"
		echo "--------------------------------------------------------------------------------------" >> "$HERE/server.log"
		stop_server && start_server
		;;
	*)
		echo "you should use either start, stop, or restart"
esac

