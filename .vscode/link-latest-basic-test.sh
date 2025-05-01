#!/bin/bash
latest="deps/"$(basename $(ls -t target/debug/deps/td3_tests-* | head -n 1))
ln -sf "$latest" target/debug/td3_tests_latest
