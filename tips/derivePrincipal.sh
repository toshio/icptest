#!/bin/bash
#
# Derive ICP Principal from ECDSA private.pem
# See https://5n2bt-lqaaa-aaaae-aajfa-cai.raw.icp0.io/
#
privatePem=$1

# check if parameter specified
if [ -z "${privatePem}" ]; then
  echo "Usage: $(basename $0) <ecdsa private pem file>" >&2
  exit 1
fi

# Check if file exists
if [ ! -f "${privatePem}" ]; then
  echo "File not found. (${privatePem}" >&2
  exit
fi

# Principal Id (raw)
rawPrincipal=$(
  openssl ec -in ${privatePem} -pubout -outform DER 2>/dev/null |
  sha224sum |
  cut -d ' ' -f 1 |
  sed s/$/02/
)

# Calc crc32
# See https://stackoverflow.com/questions/44804668/how-to-calculate-crc32-checksum-from-a-string-on-linux-bash
crc32=$(
  echo -n ${rawPrincipal} |
  xxd -r -p |
  python3 -c 'import sys;import zlib;print("{:x}".format(zlib.crc32(sys.stdin.buffer.read())%(1<<32)))'
)

# Principal
principal=$(
  echo "${crc32}${rawPrincipal}" |
  xxd -r -p |
  base32 |
  tr A-Z a-z |
  tr -d "=" |
  sed -E 's/(.{5})/\1-/g'
)

echo ${principal}
