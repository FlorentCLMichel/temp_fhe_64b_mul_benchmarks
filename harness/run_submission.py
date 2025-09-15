#!/usr/bin/env python3

# Copyright (c) 2025 HomomorphicEncryption.org
# All rights reserved.
#
# This software is licensed under the terms of the Apache v2 License.
# See the LICENSE.md file for details.

"""
run_submission.py - run the entire submission process, from build to verify
"""

import subprocess
import utils
from params import instance_name
from pathlib import Path
from sys import platform

IMPL_DIR = "implementation_0_tfhe_rs"

def main() -> int:
    """
    Run the entire submission process, from build to verify
    """

    if not platform.startswith('linux'):
        print(f"It appears you're running on {platform}; only linux is supported")
        return 1

    # 0. Prepare running
    # Get the arguments
    size, params, seed, num_runs, clrtxt = utils.parse_submission_arguments('Run the add-two-values FHE benchmark.')
    test = instance_name(size)
    print(f"\n[harness] Running submission for {test} dataset")

    # Ensure the required directories exist
    utils.ensure_directories(params.rootdir)

    # Build the submission if needed
    print("Build the submission executable...")
    utils.build_submission(params.rootdir/"scripts")

    # The harness scripts are in the 'harness' directory,
    # the executables are in the directory implementation_0_tfhe_rs/target/release
    harness_dir = params.rootdir/"harness"
    exec_dir = params.rootdir/"implementation_0_tfhe_rs"/"target"/"release"

    # Remove and re-create IO directory
    io_dir = params.iodir()
    if io_dir.exists():
        subprocess.run(["rm", "-rf", str(io_dir)], check=True)
    io_dir.mkdir(parents=True)
    utils.log_step(0, "Init", True)

    # 1. Client side: Generate the datasets by running the cleartext implementation
    print("Generate the input and expected data...")
    cmd = ["python3", harness_dir/"cleartext_impl.py", str(size)]
    if seed is not None:
        cmd.extend(["--seed", str(gendata_seed)])
    subprocess.run(cmd, check=True)
    utils.log_step(1, "Dataset generation")
    
    # 2. Client side: Generate the keys
    Path("temp").mkdir(exist_ok=True)
    cmd = [IMPL_DIR + "/target/release/run_gen_keys"]
    subprocess.run(cmd, check=True)
    utils.log_step(2, "Key generation")
    
    # 3. Client side: Encode and encrypt the dataset
    cmd = [IMPL_DIR + "/target/release/run_encrypt", instance_name(size)]
    subprocess.run(cmd, check=True)
    utils.log_step(3, "Encryption")
    
    # 4. Server side: Run the encrypted processing
    # TODO 
    utils.log_step(4, "Homomorphic mul")
   
    # 5. Client side: Decrypt and check the result
    # TODO 
    utils.log_step(5, "Decryption + check the results")
    
    # 6. Store measurements
    # TODO 

    print(f"\nAll steps completed for the {instance_name(size)} dataset!")
    return 0


if __name__ == "__main__" :
    main()
