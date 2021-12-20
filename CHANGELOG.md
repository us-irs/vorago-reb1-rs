Change Log
=======

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [unreleased]

## [v0.3.2]

- Bump HAL dependency to v0.5.0. Changed API, especially for IRQ handling

## [v0.3.1]

- Updated ADC code and dependency

## [v0.3.0]

- Completed baseline features to support all sensors on the REB1 sevice
- Relicensed as Apache-2.0 and moved to https://egit.irs.uni-stuttgart.de/rust/vorago-reb1

## [v0.2.3]

- Added basic accelerometer example. Board in not populated so it is not complete, but
  it provides a starting point
- Added ADC base library and example building on the new max116xx-10bit device driver crate
