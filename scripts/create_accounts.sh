#!/bin/sh

set -e

near create-account stdcon.$MASTER_ACCOUNT --masterAccount $MASTER_ACCOUNT
near create-account nostdcon.$MASTER_ACCOUNT --masterAccount $MASTER_ACCOUNT
