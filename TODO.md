## To Do

* [x] Add integration tests (at least a few examples)
* [ ] Populate the default rules with some common values (py, js, ts, terraform etc)
* [x] Add `clippy` to the CI
* [ ] Add `lintrule` to the CI
* [x] Add `cargo fmt` to the CI
* [ ] Add `cargo doc` to the CI
* [x] Publish the crate on tag
  * [ ] Start a page with supported selectors and rules by default
* [ ] Add docs about how to run locally and in CI
* [ ] Add tests on common examples
* [ ] Add a cli option to check if all the packages are monitored. It can be a simple generate new config
      deep compare with what is in the repository
* [ ] Consider adding extra detector types
  * [ ] HAS_FILE
  * [ ] HAS_DIRECTORY_REGEX -> can be used to detect if we are in a git repo or have a .github directory
  * [ ] HAS_FILE_WITH_CONTENT_MATCHING
  * [ ] HAS_FILE_DEEP_PATH
* [ ] Question: should we remove `type` and rely on a name only ?
