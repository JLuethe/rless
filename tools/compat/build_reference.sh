#!/usr/bin/env bash
# Reproduce the reference workflow; native C/autoconf/terminal development tools required.
set -euo pipefail
reference_dir=${1:?Usage: build_reference.sh /external/reference/path [configure arguments ...]}
shift
project_dir=$(cd -- "$(dirname -- "$0")/../.." && pwd)
reference_dir=$(realpath -m -- "$reference_dir")
case "$reference_dir/" in "$project_dir/"*) echo 'Reference must remain outside the rless checkout.' >&2; exit 1;; esac
reference_sha=7f46e9ff46216f3e560cd1f41ddf3b36b11eec0e
if [[ ! -e "$reference_dir" ]]; then
  git clone --no-checkout https://github.com/gwsw/less.git "$reference_dir"
  git -C "$reference_dir" checkout --detach "$reference_sha"
fi
[[ $(git -C "$reference_dir" rev-parse HEAD) == "$reference_sha" ]] || { echo 'Reference commit mismatch.' >&2; exit 1; }
git -C "$reference_dir" diff --quiet || { echo 'Reference has tracked edits; preserve and resolve them first.' >&2; exit 1; }
git -C "$reference_dir" diff --cached --quiet || { echo 'Reference has staged edits; preserve and resolve them first.' >&2; exit 1; }
cd -- "$reference_dir"
make -f Makefile.aut all
./configure --with-regex=gnu "$@"
make -j2
./less --version
comparison_file=$(mktemp /tmp/rless-reference-copy.XXXXXX)
trap 'rm -f -- "$comparison_file"' EXIT
./less README > "$comparison_file"
cmp README "$comparison_file"
# Pass configured flags to the independent lesstest makefile as well.
reference_cflags=$(sed -n 's/^CFLAGS = //p' Makefile)
reference_cppflags=$(sed -n 's/^CPPFLAGS = //p' Makefile)
reference_ldflags=$(sed -n 's/^LDFLAGS = //p' Makefile)
make check CFLAGS="$reference_cflags $reference_cppflags" LDFLAGS="$reference_ldflags"
