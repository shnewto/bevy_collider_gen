# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0](https://github.com/shnewto/bevy_collider_gen/compare/0.2.2...0.3.0) - 2024-11-20

### Added

- Feature `parallel` for parallel work with data
  ([594e767](https://github.com/shnewto/bevy_collider_gen/commit/594e767586494e821009f18fe8cbd96fe1b3703e)).
- [`rayon`] dependency for `parallel` features
  ([594e767](https://github.com/shnewto/bevy_collider_gen/commit/594e767586494e821009f18fe8cbd96fe1b3703e))
  ([1019ba6](https://github.com/shnewto/bevy_collider_gen/commit/1019ba697f0c3b20a633eaa55af7446193ab6e9e)).
- `ColliderType` enumeration
  ([618b5f7](https://github.com/shnewto/bevy_collider_gen/commit/618b5f7d081744a9f1eafee10ce9c0d21b95e1a9)).
- Functions `generate colliders`, `generate collider` for code maintainability
  ([618b5f7](https://github.com/shnewto/bevy_collider_gen/commit/618b5f7d081744a9f1eafee10ce9c0d21b95e1a9))
  ([56f3a2f](https://github.com/shnewto/bevy_collider_gen/commit/56f3a2faebab3190c170ecf68e2067fa51b1ce1c)).

### Changed

- Upgrade dependencies: [`edges`] 0.4
  ([e4c501f](https://github.com/shnewto/edges/commit/e4c501fa701a47c9ac67bd17e805ede77ad6485a)).
- The `rapier2d`, `parallel` features are enabled and `avian2d` are disabled by default
  ([f000104](https://github.com/shnewto/bevy_collider_gen/commit/f0001048d5000b34ef888fca76ccd26f3edeb3e9)).
- Dependencies [`bevy`] replaced by [`bevy_math`]
  ([d2079cb](https://github.com/shnewto/bevy_collider_gen/commit/d2079cb293d6aad43588ef849539c2e885de3e0c))
  ([42c750e](https://github.com/shnewto/bevy_collider_gen/commit/42c750ecdd9b4d8f167a20e204a6692ea1cd6bee)).
- `Edges` structure now in public external crate [`edges`]
  ([618b5f7](https://github.com/shnewto/bevy_collider_gen/commit/618b5f7d081744a9f1eafee10ce9c0d21b95e1a9)).

### Removed

- [`thiserror`] from dependencies
  ([2f1f35b](https://github.com/shnewto/bevy_collider_gen/commit/2f1f35b4f6275ad079b1fe76e1a976ba6a2c3b04)).
- Functions are removed for code maintainability
  ([618b5f7](https://github.com/shnewto/bevy_collider_gen/commit/618b5f7d081744a9f1eafee10ce9c0d21b95e1a9)):
  - `single_polyline_collider_translated`
  - `single_polyline_collider_raw`
  - `single_convex_polyline_collider_translated`
  - `single_convex_polyline_collider_raw`
  - etc.

## [0.2.2](https://github.com/shnewto/bevy_collider_gen/compare/0.2.1...0.2.2) - 2024-08-14

### Added

- Feature `avian2d` and enabled by default for support [`avian2d`] colliders
  ([9cd6ac9](https://github.com/shnewto/bevy_collider_gen/commit/9cd6ac9f362fa867e6d1bf38b4f8681ac9d09754)).

### Changed

- Upgrade dependencies: [`bevy`] 0.14, [`bevy_prototype_lyon`] 0.12
  ([9cd6ac9](https://github.com/shnewto/bevy_collider_gen/commit/9cd6ac9f362fa867e6d1bf38b4f8681ac9d09754)).

### Removed

- Feature `xpbd_2d` and support for [`bevy_xpbd_2d`] colliders
  ([9cd6ac9](https://github.com/shnewto/bevy_collider_gen/commit/9cd6ac9f362fa867e6d1bf38b4f8681ac9d09754)).

## [0.2.1](https://github.com/shnewto/bevy_collider_gen/compare/0.2.0...0.2.1) - 2024-05-13

### Changed

- Upgrade dependencies: [`edges`] 0.3.2
  ([cbb8c5c](https://github.com/shnewto/bevy_collider_gen/commit/cbb8c5c1474f08bed0b405c76da3f99bd2b27540)).

## [0.2.0](https://github.com/shnewto/bevy_collider_gen/compare/0.1.0...0.2.0) - 2024-03-04

### Added

- Dependencies: [`thiserror`], [`edges`]
  ([01dc9be](https://github.com/shnewto/bevy_collider_gen/commit/01dc9be747fb971d3222702d203eb471d5b156d7)).
- `Edges` structure from [`edges`] crate
  ([01dc9be](https://github.com/shnewto/bevy_collider_gen/commit/01dc9be747fb971d3222702d203eb471d5b156d7)).

### Removed

- Functions ([01dc9be](https://github.com/shnewto/bevy_collider_gen/commit/01dc9be747fb971d3222702d203eb471d5b156d7)):
  - `image_to_edges`
  - `march_edges`
  - `multi_image_edge_translated`
  - `multi_image_edges_raw`
  - `single_image_edge_raw`
  - `single_image_edge_translated`
  - `translate_vec`

[`bevy`]: https://crates.io/crates/bevy
[`bevy_math`]: https://crates.io/crates/bevy_math
[`avian2d`]: https://crates.io/crates/avian2d
[`rayon`]: https://crates.io/crates/rayon
[`edges`]: https://crates.io/crates/edges
[`thiserror`]: https://crates.io/crates/thiserror
[`bevy_prototype_lyon`]: https://crates.io/crates/bevy_prototype_lyon
[`bevy_xpbd_2d`]: https://crates.io/crates/bevy_xpbd_2d