# DyseOS Development Kit

This project is mostly copied from https://github.com/embedded-rust/rust/raspberrypi-OS-tutorials.git.

Shout out Andre Richter.

The key differences are:
 
 - No more Makefiles, builds are done in docker by a bash script

  > Docker images (devbox) are built by docker-compose services and provides cargo, llvm and qemu \
  > Docker compose services configure the workspace for specific boards \
  > Published devboxes can be used to build the published kernel or another cargo project \
  > The entrypoint provides a single command to run many different cargo tools \
     
     docker compose run <SERVICE> <OPTIONS> <CMD>

  > The command will be executed by the entrypoint before exiting (set to 'bash' for an interactive terminal)

```
     Devbox Usage
     
  -c Clean the workspace
  -d Dump kernel elf info
  -b Cargo binary to build, from Cargo.toml
  -n Don't Build the elf or run cargo objcopy
  -q Run QEMU emulator, requires that bin is the kernel image
  -g Run QEMU emulator and attach LLDB session, overrides -q
  -p <PROFILE> Cargo profile to build
  -f <FEATURES> Cargo features to enable
```

```
     # Example
# clean, build and dump info on the rpi kernel, then exit
docker compose run rpi -d -c -p debug exit
```

```
     # Example
# no build, no dump, run qemu attached to debugger, then start bash terminal
docker compose run rpi -n -g bash
```

```
     # Example
# no build, clean, dump and then print the crates structure
# the rust dump tools will also build the project so this is safe
docker compose run rpi -n -d cat <BIN>_structure.txt
```
     
 - On a push to master Github actions will publish new docker images
 - Devboxes use lldb as the debugger not gdb
