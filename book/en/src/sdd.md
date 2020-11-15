# Software Design Description

## Goals

Describe the design choices and structures found in Real-Time
Interrupt-driven Concurrency (RTIC)

## Functional Description

RTIC is a framework translating specially annotated Rust code to analyse,
verify and generate executable code.

## User Interface

An attribute macro applied on top of a Rust module, where the module contains
annotated functions and structures.
