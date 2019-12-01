# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    #cross rustc --bin ssb-patchql --target $TARGET --release -- -C lto
    cross rustc --manifest-path http/Cargo.toml --target $TARGET --release -- -C lto
    cross rustc --manifest-path stdio/Cargo.toml --target $TARGET --release -- -C lto

    cp target/$TARGET/release/main $stage/http_server
    cp target/$TARGET/release/jsonrpc_stdio $stage/jsonrpc_stdio

    cd $stage
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
