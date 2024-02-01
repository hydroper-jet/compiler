# Symbol operations

## Property resolution

* [ ] Property resolution throws `PropertyResolutionError` error in specific occasions.
* [ ] When extends_class.is_unresolved() is true for a resolvee class, resolution throws `PropertyResolutionError::DeferVerification`.
* [ ] When extends_interface[i].is_unresolved() is true for a resolvee interface, resolution throws `PropertyResolutionError::DeferVerification`.
* [ ] When implements[i].is_unresolved() is true for a resolvee class, resolution throws `PropertyResolutionError::DeferVerification`.
* [ ] When the static type of a property (`property_static_type()`) to be returned from the resolution is `Unresolved`, throw `PropertyResolutionError::DeferVerification`.