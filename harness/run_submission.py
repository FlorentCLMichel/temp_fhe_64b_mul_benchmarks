"""
run_submission.py - run the entire submission process, from build to verify
"""

import subprocess
import utils
from params import instance_name

def main():
    """
    Run the entire submission process, from build to verify
    """

    # 0. Prepare running
    # Get the arguments
    size, params, seed, num_runs, clrtxt = utils.parse_submission_arguments('Run the add-two-values FHE benchmark.')
    test = instance_name(size)
    print(f"\n[harness] Running submission for {test} dataset")

    # Ensure the required directories exist
    utils.ensure_directories(params.rootdir)

    # Build the submission if needed
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

    # 1. Generate the datasets by running the cleartext implementation
    print("Generate the input and expected data...")
    cmd = ["python3", harness_dir/"cleartext_impl.py", str(size)]
    if seed is not None:
        cmd.extend(["--seed", str(gendata_seed)])
    subprocess.run(cmd, check=True)
    utils.log_step(1, "Dataset generation")


if __name__ == "__main__" :
    main()
