## Unreleased

## 5.4.1 (29 Jan 2023)

- link libresolv on macos

## 5.4.0 (28 Jan 2023)

- c-ares 1.19.0

## 5.3.4 (25 Apr 2021)

- emit `cargo:include` from build script

## 5.3.3 (6 Nov 2021)

- fix to the build-cmake feature

## 5.3.2 (6 Nov 2021)

- feature "build-cmake" to use the cmake-based build for c-ares

## 5.3.1 (27 Oct 2021)

- c-ares 1.18.1

## 5.3.0 (26 Oct 2021)

- c-ares 1.18.0

## 5.2.0 (23 Aug 2021)

- `cargo diet` to reduce crate size
- Take c-ares 1.17.2

## 5.1.0 (29 Nov 2020)

- Take c-ares 1.17.1

## 5.0.0 (15 Aug 2020)

- pull upstream c-ares - their release 1.16.1
- switch to using `RawSocket` on windows

## 4.2.0 (2 Nov 2018)

- pull upstream c-ares - their release 1.15.0

## 4.1.5 (1 Jul 2018)

- pull upstream c-ares

## 4.1.4 (30 May 2018)

- pull upstream c-ares
  - in particular, their [#191](https://github.com/c-ares/c-ares/pull/191)

## 4.1.3 (12 May 2018)

- Arrange that build output all goes to `$OUT_DIR`

## 4.1.2 (7 Apr 2018)

- Bump c-types dependency (fixes minimal-versions build on OSX)

## 4.1.1 (7 Apr 2018)

- Bump metadeps dependency (fixes minimal-versions build)

## 4.1.0 (16 Feb 2018)

- pull upstream c-ares - their release 1.14.0
- have a few more functions take `const` channel
  - `ares_save_options`, `ares_timeout`, `ares_get_servers`,
    `ares_get_servers_ports`
- start maintaining a CHANGELOG
