#!/bin/sh -ex

BASEDIR=$(dirname "$0")
SCHEMA_DIR="$BASEDIR/../e2e-server/proto"
OUTDIR="$BASEDIR/_gen"

# cleanup
rm -rf $OUTDIR
mkdir -p $OUTDIR

# We need both the js output and the grpc-web output since the grpc-web
# generator doens't include the protobuf core transport itself (or something).
# More details here:
#   https://github.com/grpc/grpc-web#typescript-support
protoc -I $SCHEMA_DIR $SCHEMA_DIR/chatroom.proto  \
  --js_out=import_style=commonjs,binary:$OUTDIR \
  --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:$OUTDIR
