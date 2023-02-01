#!/bin/sh

set -e

near delete --force true stdcon.$MASTER_ACCOUNT $MASTER_ACCOUNT --verbose true
near delete --force true nostdcon.$MASTER_ACCOUNT $MASTER_ACCOUNT --verbose true
