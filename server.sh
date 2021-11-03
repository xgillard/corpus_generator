:
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

export ROCKET_PROFILE=release

# just start the server
function start_server () {
	nohup ./corpus -c './public/corpus' &
	echo "server started"
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
		| xargs -L 1 -J % kill %
	echo "server stopped"
}

case $1 in
	"start")
		start_server
		;;
	"stop")
		stop_server
		;;
	"restart"):
		stop_server && start_server
		;;
	*)
		echo "you should use either start, stop, or restart"
esac

