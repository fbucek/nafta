# Changes

## [0.1.3] - 2020-05-10

### Changed

* Added unitest for tempfile
* Known issue #9 test randomly fail on GitHub Actions Windows Server build
   - Not possible to reproduce bug on local Windows.

## [0.1.2] - 2020-05-08

### Changed

* Updated documentation

## [0.1.1] - 2020-05-08

### Changed

* Dependancy `tempdir` ( deprecated ) -> `tempfile` [[#7]]
* Fixed issue where on Windows `temp` dir was not removed

[#7]: https://github.com/fbucek/nafta/issues/7
