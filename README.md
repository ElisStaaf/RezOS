# RezOS
A modern operating system written in the rust programming language.
### premise:
- Rust as a primary language for safety, speed and security
- Modern bootloader support with the limine protocol
- Architectural compatibility for x86_64 and AARCH64/ARM64
- Innovative approach to Micro/Monolithic kernel design
- Graph VFS
- Capability based permissions
- Proper async IO interface
- Running DOOM
- Full network & graphics stack

# Setup instructions
## A. download the source code
1. Clone the source repository <br>
`git clone https://www.github.com/GandelXIV/RezOS.git`
2. Update git submodules <br>
`git submodule update --init --progress`
## B. Prepare the environment
Install dependencies, there are 2 ways of doing this:
### Locally
  This option is only viable on linux, so if you are on a different platform consider using the Docker method. <br> <br>
  Use an install script located in `scripts/install/all/{your-distro}.sh`. <br>
  If your distro does not have a script, you can still use `scripts/install/rust-linux.sh` to setup rust, then install all the packages in `scripts/install/packages/packages.md` manually
###
### Inside Docker
  This option is cross-platform and does not polute your system with pkgs, however is harder to operate.
  1. Install [docker](https://www.docker.com/)
  2. Build the container as described in `docker/setup.sh` or `scripts/docker/setup.sh`
  3. Before compiling, start the container environment with `docker/run.sh` or `scripts/docker/run.sh`. Once the environment is up and running, `cd` into `/home/rezos` and carry on with compiling.
###
## C. Configure
One last step before compiling is to configure the project, you need to do this only once. <br> Either run `./config.sh` or follow the steps in `config/README.md`.
##
## D. Compile!
Run `python3 smeltfile.py iso`, which will generate the final image in `build/image.iso`. You can start the OS in qemu with `qemu-system-x86_64 build/image.iso -serial file:serial.log`.
