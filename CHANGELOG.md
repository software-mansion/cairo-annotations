# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.2] - 2025-02-20

### Added

- `KeccakRound` to `DeprecatedSyscallSelector` (introduced in starknet 0.13.4)

## [0.2.1] - 2024-11-27

### Added

- `GetClassHashAt` to `DeprecatedSyscallSelector` (introduced in starknet 0.13.3)

## [0.2.0] - 2024-11-22

### Added

- removed syscall_counter from ExecutionResources
- wrapped CallTraceV1 into Box for better performance
- implemented method `DeprecatedSyscallSelector::all()` for getting all possible libfuncs
- added traits for `DeprecatedSyscallSelector` to allow its conversion from/to a string
- added trait for `FunctionName` to allow its generation from sierra statement idx
