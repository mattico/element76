element76
=========

[![Build Status](https://travis-ci.org/mvdnes/element76.png?branch=master)](https://travis-ci.org/mvdnes/element76)

An operating system written in Rust

Build instructions
------------------

If you are not on i686:

- run `./downloadrustlibdir.sh` to download the i686 nightly libraries

Compile:

- `make`

Run:

- `make run`

Changes From mvdnes/element76
-----------------------------

The way I've got this set-up is that `mattico/master` tracks `mvdnes/master` (for now) so that I can keep in sync.  Most of the changes I've made since deciding on this system are on the `/dev` branch, where I've changed the build system a little bit to add a (not yet working) x86_64 target among other things.  On `/dev` check the Makefile for new build targets. All of the other branches have /dev as their base.  
`/serial` has a working serial driver which is close to being fully featured.  Use `ESC-2` in QEMU to see the serial console. `/libcpuid` was an attempt to use rust-libcpuid for cpuid stuff that I stopped when I realized that I'd need to make a lot of changes to deal with `#![no_std]`. 
`/oo-cpuid` is an attempt to make a rust-native cpuid library that has gone through a lot of orginizational upheaval while I was trying to think of the best interface to expose. Now, I figure that I'll use a much simpler set of functions for just the stuff the kernel needs and let a more fully featured cpuid live in userspace.  
`/pae` is an obsolete branch where I was going to implement PAE.  Most of this stuff is pretty half-baked for now.  Maybe I'll get around to it soon.
