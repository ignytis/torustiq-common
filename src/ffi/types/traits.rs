/// Returns a shallow copy of the object.
/// Used to pass a copy of original object to external (shared object) function,
/// but keep ownership of original object inside caller method.
/// A typical workflow:
/// let o = module.get_object();
/// do_something(o.shallow_copy());
/// module.free_object(o);
pub trait ShallowCopy {
    fn shallow_copy(&self) -> Self;
}