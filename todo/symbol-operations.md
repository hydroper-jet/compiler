# Symbol operations

## Property resolution

* [ ] Property resolution throws `PropertyResolutionError` error in specific occasions.
* [ ] When extends_class.is_unresolved() is true for a resolvee class, resolution throws `PropertyResolutionError::Defer`.
* [ ] When extends_interface[i].is_unresolved() is true for a resolvee interface, resolution throws `PropertyResolutionError::Defer`.
* [ ] When implements[i].is_unresolved() is true for a resolvee class, resolution throws `PropertyResolutionError::Defer`.