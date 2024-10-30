# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- removed syscall_counter from ExecutionResources
- wrapped CallTraceV1 into Box for better performance
- implemented method `DeprecatedSyscallSelector::all()` for getting all possible libfuncs
- implemented traits for `DeprecatedSyscallSelector` to allow its conversion from/to a string
