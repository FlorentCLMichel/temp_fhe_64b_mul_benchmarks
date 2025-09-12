import argparse
from params import InstanceParams, TOY, LARGE
from typing import Tuple

def parse_submission_arguments(workload: str) -> Tuple[int, InstanceParams, int, int, int]:
    """
    Get the arguments of the submission. Populate arguments as needed for the workload.
    """
    # Parse arguments using argparse
    parser = argparse.ArgumentParser(description=workload)
    parser.add_argument('size', type=int, choices=range(TOY, LARGE+1),
                        help='Instance size (0-toy/1-small/2-medium/3-large)')
    parser.add_argument('--num_runs', type=int, default=1,
                        help='Number of times to run steps 4-9 (default: 1)')
    parser.add_argument('--seed', type=int,
                        help='Random seed for dataset generation')
    parser.add_argument('--clrtxt', type=int,
                        help='Specify with 1 if to rerun the cleartext computation')

    args = parser.parse_args()
    size = args.size
    seed = args.seed
    num_runs = args.num_runs
    clrtxt = args.clrtxt

    # Use params.py to get instance parameters
    params = InstanceParams(size)
    return size, params, seed, num_runs, clrtxt
