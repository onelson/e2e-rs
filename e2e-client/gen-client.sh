#!/bin/sh -ex

BASEDIR=$(dirname "$0")
SCHEMA_DIR="$BASEDIR/../e2e-server/proto"
OUTDIR="$BASEDIR/_gen"

# cleanup
rm -rf $OUTDIR
mkdir -p $OUTDIR

# do the codegen
protoc -I $SCHEMA_DIR $SCHEMA_DIR/chatroom.proto  \
  --js_out=import_style=commonjs,binary:$OUTDIR \
  --grpc-web_out=import_style=commonjs+dts,mode=grpcwebtext:$OUTDIR
