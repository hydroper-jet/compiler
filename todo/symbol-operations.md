# Symbol operations

## Property resolution

* [x] Property resolution throws `PropertyResolutionError` error in specific occasions.
* [x] When extends_class.is_unresolved() is true for a resolvee class, resolution throws `PropertyResolutionError::DeferVerification`.
* [x] When extends_interface[i].is_unresolved() is true for a resolvee interface, resolution throws `PropertyResolutionError::DeferVerification`.
* [x] When implements[i].is_unresolved() is true for a resolvee class, resolution throws `PropertyResolutionError::DeferVerification`.
* [x] When the static type of a property (`property_static_type()`) to be returned from the resolution is `Unresolved`, throw `PropertyResolutionError::DeferVerification`.
* [x] Throw when a value's base is `void` or nullable