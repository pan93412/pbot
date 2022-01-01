#!/usr/bin/env bash
# -*- coding: utf-8 -*-

cargo_f_check() {
    features=$1
    # human-readable features
    hr_features=$features

    if [[ "$hr_features" == "" ]]; then
        hr_features="No Features"
    fi
    
    printf "\n\n-----\n\x1b[1mTesting: \x1b[33m%s\x1b[0m\n-----\n\n" "$hr_features"

    if cargo check --no-default-features --features "$features"
    then
        printf "\n\x1b[1;32m ---- SUCCEED ---- \x1b[0m\n\n"
    else
        printf "\n\x1b[1;31m ---- FAILURE ---- \x1b[0m\n\n"
        exit 1
    fi
}

true \
    && cargo_f_check "" \
    && cargo_f_check fwdmod \
    && cargo_f_check getinfomod \
    && cargo_f_check "fwdmod getinfomod"
