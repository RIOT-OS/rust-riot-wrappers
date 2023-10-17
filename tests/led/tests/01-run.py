#!/usr/bin/env python3

import os
import sys
from testrunner import run

def test(child):
    # Both LEDs light up at some point
    child.expect("LED_RED_TOGGLE")
    child.expect("LED_GREEN_TOGGLE")

if __name__ == "__main__":
    if os.environ['BOARD'] != 'native':
        print("Automated test only works on native (other boards don't report hteir LED activity)", file=sys.stderr)
        sys.exit(1)
    sys.exit(run(test))
