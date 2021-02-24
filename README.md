# superorganism
Community-driven community

This repository contains the PoC-implementation of my masters thesis. It shows that a community can completely manage and evolve themselves without central
authorities. Although the community does use a community treasure to finance the projects they have selected, money does have no weight in the decisions,
id est members can not use money to gain more influence. In future development, any project that was voted for and realized by the community is maintained
by the community and funded by the system. This means that projects which were realized by the community, are free to use for the community. Whether projects
are kept alive lays completly in the hands of the community. Services that the community has no interest in anymore will eventually die out (although they can
be "revived" at any time if the community decides to do so).

To experience this system, you can either run a pre-built vm (1) or build everything from the sources and run it directly on your machine (2).

1 - Using the pre-built VM
--------------------------
1) Install VirtualBox
2) Extract Superorganism.vdi.xz, you can retrieve it from IPFS:
   - Direct: ipfs://QmPHTjEYS8J8gfQJZ9gzq1bGLmjkT2Gb5jD19znLvtjzSV?filename=Superorganism.vdi.xz
   - Gateway 1: https://ipfs.io/ipfs/QmPHTjEYS8J8gfQJZ9gzq1bGLmjkT2Gb5jD19znLvtjzSV?filename=Superorganism.vdi.xz
   - Gateway 2: https://dweb.link/ipfs/QmPHTjEYS8J8gfQJZ9gzq1bGLmjkT2Gb5jD19znLvtjzSV?filename=Superorganism.vdi.xz
3) Create a new virtual machine in VirtualBox and insert Superorganism.vdi as the virtual hard drive
4) Configure processor usage, RAM, VRAM, etc.
5) Run the virtual machine
6) Double click on the desktop item "run-blockchain.sh" to start the Superorganism ecosystem
7) Double click on the desktop item "run-frontend.sh" and wait for the console log that ends with "HtmlWebpackPlugin_0 [built]"
8) Launch firefox, it should display the webpage at localhost:3000
9) Interact with the system.

2 - Build everything from sources
---------------------------------
1) Install rustup (https://rustup.rs/)
2) Install the nightly compiler toolchain: rustup toolchain install nightly-2020-10-05
3) List rust toolchains: rustup show
4) Copy string containing "nightly-2020-10-05", for example "nightly-2020-10-05-x86_64-unknown-linux-gnu"
5) Select the copied string as the default toolchain: rustup default <copied toolchain string>
6) Install WebAssembly target: rustup target add wasm32-unknown-unknown
7) Switch folder to implementation/superorganism
8) Run: cargo build --release (note: needs packages clang, llvm, build-essential, git)
9) While it builds, get yarn 1.22.5 (https://github.com/yarnpkg/yarn/releases/tag/v1.22.5) and node v12.19.0 (https://nodejs.org/en/blog/release/v12.19.0/)
10) Switch folder to implementation/superorganism-frontend
11) Get dependencies by running: yarn
12) When "cargo build --release" has finished, run "cargo run --release" in the same console
13) When "yarn" has finished and "cargo run --release" is running, execute "yarn run start" in the same console where "yarn" was executed
14) Launch firefox and browse the web page at localhost:3000
15) Interact with the system.
