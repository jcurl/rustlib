/// Represents a buffer, either owning or referenced.
///
/// When using the trait [BinParser], the implementation may have a reference to
/// the buffer given to it from outside, or it might source the buffer somewhere
/// else. Returning a slice `&[u8]` is easy for the case when the struct has a
/// reference to the data, but if it needs to load it from a file, then it must
/// somehow allocate memory in a safe way that doesn't leak. References aren't
/// owning, se we need something as a container. The [Buffer] enumeration is
/// such a container that abstracts where the buffer came from, so when it's
/// dropped, the reference is lost, or the memory is freed.
#[derive(Debug)]
pub(crate) enum Buffer<'b> {
    AsRef(&'b [u8]),
    Owning(Vec<u8>),
}

#[allow(dead_code)]
impl<'b> Buffer<'b> {
    /// Check if the buffer is a reference.
    ///
    /// Returns `true` if this enumeration is just a reference to somewhere
    /// else.
    pub(crate) const fn is_ref(&self) -> bool {
        match *self {
            Buffer::AsRef(_) => true,
            Buffer::Owning(_) => false,
        }
    }

    /// Check if the buffer is owned.
    ///
    /// Returns `true` if this enumeration is owning the buffer, that is freed
    /// when this object is dropped.
    pub(crate) const fn is_owned(&self) -> bool {
        match *self {
            Buffer::AsRef(_) => false,
            Buffer::Owning(_) => true,
        }
    }

    /// Get a reference to the underlying buffer. It's lifetime depends on the
    /// lifetime of this object.
    pub(crate) fn buffer(&'b self) -> &'b [u8] {
        match self {
            Buffer::AsRef(r) => r,
            Buffer::Owning(o) => o.as_slice(),
        }
    }
}
