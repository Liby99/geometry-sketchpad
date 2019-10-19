use super::Viewport;

pub struct Virtual<T>(pub T);

pub struct Screen<T>(pub T);

pub trait ViewportTransform : Sized {
  fn to_screen(from: Virtual<Self>, vp: Viewport) -> Screen<Self>;
  fn to_virtual(from: Screen<Self>, vp: Viewport) -> Virtual<Self>;
}