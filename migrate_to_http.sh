:
################################################################################
# The point of this script it simply to replace all occurrences of https by    #
# http in the source code of the inginious frontend of linfo2263. This is      #
# used to solve some unexpected stability issues caused by a crash of the TLS  #
# stack.                                                                       #
#                                                                              #
# Note:                                                                        #
# This script uses the command gsed instead of sed to perform the replacement. #
# This is simply because my machine is a mac, and sed does not come installed  #
# by default. Therefore I had to install gnu-sed by myself. Hence the gsed     #
# command. Apart from that, there should be absolutely no difference between   #
# vanilla sed and gsed.                                                        #
################################################################################

case $1 in
	dryrun)
		# dry run
		find /Volumes/LINFO2263/project* -type f -exec grep -i 'https://linfo2263.info.ucl.ac.be' {} \;
		;;
	proceed)
		# do it for real
		#find /Volumes/LINFO2263/project2a -type f -exec gsed -i 's/https:\/\/linfo2263.info.ucl.ac.be/http:\/\/linfo2263.info.ucl.ac.be/gI' {} \;
		gsed -i 's/http:\/\/linfo2263.info.ucl.ac.be/https:\/\/linfo2263.info.ucl.ac.be/gI' /Volumes/LINFO2263/project2a/task.yaml
		gsed -i 's/http:\/\/linfo2263.info.ucl.ac.be/https:\/\/linfo2263.info.ucl.ac.be/gI' /Volumes/LINFO2263/project2a/run.sh
		gsed -i 's/http:\/\/linfo2263.info.ucl.ac.be/https:\/\/linfo2263.info.ucl.ac.be/gI' /Volumes/LINFO2263/project2a/public/common.js
		;;
	*)
		echo "use this script with either 'dryrun' or 'proceed' as first and only arg"
		;;
esac
