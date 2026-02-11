# ASC Specification Files

This directory contains the canonical source of truth for the Avionics Safety Contract (ASC) system.

## Structure

- `contracts/` - Safety contract definitions and invariants
- `types/` - Data type specifications
- `interfaces/` - Interface specifications between components

## Format

Specifications are written in YAML format and follow a strict schema for code generation.

## Usage

The `codegen` crate reads these specifications and generates:
- Rust type definitions
- Contract verification code
- Interface implementations
- Test harnesses

## Versioning

All spec files include a version field. Breaking changes require a major version bump.
