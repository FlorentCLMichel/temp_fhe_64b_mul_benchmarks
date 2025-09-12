"""
params.py - Parameters and directory structure for the submission.
"""

from pathlib import Path

# Enum for benchmark size
TOY = 0
SMALL = 1
MEDIUM = 2
LARGE = 3

def instance_name(size):
    """Return the string name of the instance size."""
    if size > LARGE:
        return "unknown"
    names = ["toy", "small", "medium", "large"]
    return names[size]

class InstanceParams:
    """Parameters that differ for different instance sizes."""

    def __init__(self, size, rootdir=None):
        """Constructor."""
        self.size = size
        self.rootdir = Path(rootdir) if rootdir else Path.cwd()

        if size > LARGE:
            raise ValueError("Invalid instance size")

        # parameters for sizes: toy, small, medium, large
        size_bound = [10, 1000, 100000, 10000000]

        self.size_bound = size_bound[size]

    def get_size(self):
        """Return the instance size."""
        return self.size

    def get_size_bound(self):
        """Return the input (LHS and RHS) dimension."""
        return self.size_bound

    # Directory structure methods
    def subdir(self):
        """Return the submission directory of this repository."""
        return self.rootdir

    def datadir(self):
        """Return the dataset directory path."""
        return self.rootdir / "datasets" / instance_name(self.size)

    def iodir(self):
        """Return the I/O directory path."""
        return self.rootdir / "io" / instance_name(self.size)
    
    def measuredir(self):
        """Return the measurements directory path."""
        return self.rootdir / "measurements" / instance_name(self.size)
