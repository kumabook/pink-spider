#!/bin/bash
#
# A utility script for downloading the latest stable build of Rust,
# uploading it to S3, and updating your RustConfig file.  This requires the
# official 'aws' command line tool, which can be installed and configured
# as follows:
#
#   sudo pip install awscli
#   aws configure
#
# To run this script, first create an S3 bucket, and then run:
#
#   ./update-bin my-bucket-name

# Quit on the first error we encounter.
set -e

# Make sure we were passed an S3 bucket.
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <S3 bucket name>" 1>&2
    exit 1
else
    BUCKET="$1"
fi

# Format today's date appropriately.
DATE=`date '+%Y-%m-%d'`

# The names of our stable tarballs.
RUST_TARBALL=rust-1.4.0-x86_64-unknown-linux-gnu.tar.gz

# Download our tarballs.
echo "-----> Fetching stable builds"
rm -f "$RUST_TARBALL"
curl -O "https://static.rust-lang.org/dist/$RUST_TARBALL"

# Upload our tarballs to S3.
echo "-----> Uploading to S3"
aws s3 cp "$RUST_TARBALL" "s3://$BUCKET/$DATE/" --acl public-read

# Updating RustConfig.
echo "-----> Updating RustConfig"
cat <<EOF > RustConfig
URL="https://s3.amazonaws.com/$BUCKET/$DATE/$RUST_TARBALL"
VERSION="$DATE"

echo "-----> Cleaning up"
rm -f "$RUST_TARBALL"
