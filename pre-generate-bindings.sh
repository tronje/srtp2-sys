#!/bin/sh

bindgen --whitelist-function "(srtp|SRTP|srtcp|SRTCP)_.*" \
	--whitelist-type "(srtp|SRTP|srtcp|SRTCP)_.*" \
	--whitelist-var "(srtp|SRTP|srtcp|SRTCP)_.*" \
	--blacklist-item ".*(192|gcm|GCM).*" \
	bindgen.h > src/srtp.rs
