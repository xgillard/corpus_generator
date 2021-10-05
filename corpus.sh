: 
# This is a conveniency script I wrote to avoid typing in the complete
# command that requires setting up env vars and so on. It does nothing smart, 
# just launches the appropriate command.

ROCKET_PROFILE=release nohup ./target/release/corpus_generator -c './public/corpus' &