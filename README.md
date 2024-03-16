# DyseOS Development Kit

This project's source is mostly copied from https://github.com/embedded-rust/rust/raspberrypi-OS-tutorials.git.

Shout out Andre Richter.

  ## Cargo tools

  - https://github.com/rust-embedded/cargo-binutils
  - https://github.com/regexident/cargo-modules

  ## All you need is docker

  All dependencies and targets are installed in a docker image and published to dockerhub with the 
kernel source. Images (devboxes) are tagged based on their default board configuration. Docker 
compose services configure the image builds for each board. The compose services also provide default 
options and command. 

  The devboxes come with an entrypoint that allows the user to modify the options 
and command the container starts with but not the board its configured for (the board 
configuration can probably be modified through the docker environment argument).

  ### The Entrypoint
The entrypoint provides a single command to run many different cargo tools. The dockerfile installs
the entrypoint into the image and calls it when the container is started.

To start a devbox for this project call docker compose from the project root
     
     docker compose run <SERVICE> <OPTIONS> <CMD>

  > **SERVICE** is defined by the compose file 

  > **OPTIONS** configure what the entrypoint will do 

  > **CMD** called after all tools finish (can be multiple args)

<details closed><summary>Devbox Usage</summary>

<br>

```
Usage   entrypoint.bash <OPTIONS> <COMMAND>

Compose docker compose run <SERVICE> <OPTIONS> <COMMAND>

Options:
  -c Clean the workspace
  -d Dump kernel elf info
  -b Cargo binary to build, from Cargo.toml
  -n Don't Build the elf or run cargo objcopy
  -q Run QEMU emulator, requires that bin is the kernel image
  -g Run QEMU emulator and attach LLDB session, overrides -q
  -p <PROFILE> Cargo profile to build
  -f <FEATURES> Cargo features to enable
Command everything after the last option is interpreted as a bash command after the tools execute
```

</details>

Docker compose can only be used in this repo, to build an external project use
  
     docker run -it -v ${PWD}:/home/dev/<project-name> -w /home/dev/<project-name> -p 1234:1234 mdsdev0/devbox:<service-name> <OPTIONS> <CMD>

  > **-p** forwards a port in the container to one on your host (this should enable qemu to connect with a debugger outside docker, experimental) \
  > **-v** will mount your current directory \
  > **-w** sets the working directory (entry directory) to your project \
  > **-it** enables colored prints and such (might source ~/.bashrc before entrypoint) \
  > **OPTIONS** entrypoint options \
  > **CMD** post entry command + args

  ## On a push to origin master Github actions will publish new docker images

  ## LLDB is the default debugger

<details closed><summary>Devbox Examples</summary>

<br>

```
# clean, build and dump info on the rpi kernel, then exit
docker compose run rpi -d -c -p debug exit

# no build, no dump, run qemu attached to debugger, then start bash terminal
docker compose run rpi -n -g bash

# no build, clean, dump and then print the crates structure
# the rust dump tools will also build the project so this is safe
docker compose run rpi -n -d cat <BIN>_structure.txt

# run the devbox with another cargo project in debug mode, then run cargo tests
docker run -it -v ${PWD}:/home/dev/dyseos mdsdev0/devbox:rpi -d -b binary -p debug cargo test
```

</details>