import sys
import os
import subprocess

# run cargo build --release
# check if cargo build was successful
if subprocess.run(["cargo", "build", "--release"]).returncode != 0:
    print("cargo build failed")
    sys.exit(1)

# move and rename ./target/release/screen_recorder_rs.dll to ./screen_recorder/screen_recorder.pyd
os.rename("./target/release/screen_recorder_rs.dll", "./screen_recorder/screen_recorder_rs.pyd")
