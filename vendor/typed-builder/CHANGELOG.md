# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.4.1 - 2020-01-17
### Fixed
- [**BREAKING**] now state types are placed before original generic types. 
  Previously, all state types are appended to generic arguments. For example, 
  `Foo<'a, X, Y>` yields `FooBuilder<'a, X, Y, ((), ())>` **previously**, and 
  now it becomes `FooBuilder<'a, ((), ()), X, Y, >.`. This change fix compiler error 
  for struct with default type like `Foo<'a, X, Y=Bar>`. Rust only allow type 
  parameters with a default to be trailing.

## 0.4.0 - 2019-12-13
### Added
- `#![no_std]` is now supported out of the box. (You don’t need to opt into any
  features, it just works.)
- [**BREAKING**] a `default_code` expression can now refer to the values of
  earlier fields by name (This is extremely unlikely to break your code, but
  could in theory due to shadowing)
- `#[builder(skip)]` on fields, to not provide a method to set that field.
- Control of documentation:
  - `#[builder(doc = "…")]` on fields, to document the field’s method on the
    builder. Unlike `#[doc]`, you can currently only have one value rather than
    one attribute per line; but that’s not a big deal since you don’t get to
    use the `///` sugar anyway. Just use a multiline string.
  - `#[builder(doc, builder_method_doc = "…", builder_type_doc = "…",
    build_method_doc = "…")]` on structs:
    - `doc` unhides the builder type from the documentation.
	- `builder_method_doc = "…"` replaces the default documentation that
	  will be generated for the builder() method of the type for which the
	  builder is being generated.
	- `builder_type_doc = "…"` replaces the default documentation that will
	  be generated for the builder type. Implies `doc`.
	- `build_method_doc = "…"` replaces the default documentation that will
	  be generated for the build() method of the builder type. Implies
	  `doc`.

### Changed
- [**BREAKING**] Renamed the generated builder type from
  `TypedBuilder_BuilderFor_Foo` to `FooBuilder`, for improved ergonomics,
  especially when you enable documentation of the builder type.
  - Generic identifiers were also changed, from `TypedBuilder_genericType_x` to
    `__x`. This is still expected to avoid all name collisions, but is easier
    to read in the builder type docs if you enable them.
  - Renamed the conversion helper trait for documentation purposes
    (`TypedBuilder_conversionHelperTrait_Foo` to `FooBuilder_Optional`), and
    its method name for simpler code.
- [**BREAKING**] `default_code` is now lazily evaluated instead of eagerly; any
  side-effects that there might have been will no longer occur. As is usual in
  this release, this is very unlikely to affect you.
- The restriction that there be only one `#[builder]` attribute per field has
  been lifted. You can now write `#[builder(skip)] #[builder(default)]` instead
  of `#[builder(skip, default)]` if you want to. As was already the case,
  latest definition wins.
- [**BREAKING**] Use a single generic parameter to represent the builder type's
  state (see issue #21). Previously we would use a parameter for each field.

### Changed
- Move to dual license - MIT/Apache-2.0. Previously this project was just MIT.

## 0.3.0 - 2019-02-19
### Added
- `#[builder(default_code = "...")]` syntax for defaults that cannot be parsed
  as attributes no matter what.

### Changed
- Move the docs from the crate to the custom derive proc macro.

## 0.2.0 - 2019-02-06
### Changed
- Upgraded `syn` version to support Rust 2018.
- [**BREAKING**] Changed attribute style to `#[builder(...)]`:
  - `#[default]` -> `#[builder(default)]`
  - `#[default=...]` -> `#[builder(default=...)]`
- [**BREAKING**] `default` no longer needs to be a string.
  - But you need to change your code anyways because the attribute style was changed.

## 0.1.1 - 2018-07-24
### Fixed
- Allow missing docs in structs that derive `TypedBuilder`.

## 0.1.0 - 2017-10-05
### Added
- Custom derive for generating the builder pattern.
- All setters are accepting `Into` values.
- Compile time verification that all fields are set before calling `.build()`.
- Compile time verification that no field is set more than once.
- Ability to annotate fields with `#[default]` to make them optional and specify a default value when the user does not set them.
- Generates simple documentation for the `.builder()` method.
