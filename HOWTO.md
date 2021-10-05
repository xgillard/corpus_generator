# How to configure/maintain this VM
Hi, if you are landing on this page, it is probably because you are in charge of maintaining 
the service and I left the department. (Yes, it does feel like a post mortem letter in a 
cheesy movie... and hopefully I will have graduaded and defended my thesis :fingerscrossed:)

Anyway, you are in charge of this machine and I am gone. So how do you keep it running ?
Luckily for you, I readily took care of most of the steps. 

## If you need to update the corpora 
The simplest maintenance operation you might want to do, is to add edit or remove the corpora
that are sampled to provide students with a custom dataset. If this is what you need to do, 
all it takes is to add/remove/edit the desired file in `/root/bin/public/corpus`. You don't
even need to restart the server. New content will be used immediately.

## If you need to edit the code of the server
This is quite unlikely, but in the event where you would need to adapt the code of the server,
you can either edit it immediately from `/root/gen` or you can clone it from 
`https://github.com/xgillard/corpus_generator`. The readme at the root of the repo should be 
pretty comprehensive. But here are the few caveats I thinkg of right now:

1. You will need to have the rust toolchain installed on your machine
2. You will need to compile the code before being able to actually run it

Once your edits are done and you have uploaded the source on this VM you will want to:
1. Shut the machine down (`ps alx | grep corpus` and then kill the process you found)
2. Copy the compiled artifact in `/root/bin` (`cp target/release/corpus_generator /root/bin`)
3. Restart the server

## If HTTPS is no longer working
Chances are that it is due to the fact that the certificates have expired. In that case, 
you will need to renew them. If you don't know how it is done, ask the sysadmins.

### Note: 
Use the following command to generate a fresh certificate request that admins will sign. 
```
openssl req -new -newkey rsa:2048 -nodes -keyout linfo2263.key -out linfo2263.csr
```

--
I think that this is about it. 
Best of luck,
-- Xav --

