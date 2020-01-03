use super::*;
use crate::math::*;
use crate::utilities::*;

pub trait ToVirtual {
  type Output;
  fn to_virtual(self, vp: &Viewport) -> Self::Output;
}

pub trait ToScreen {
  type Output;
  fn to_screen(self, vp: &Viewport) -> Self::Output;
}

impl ToVirtual for ScreenScalar {
  type Output = VirtualScalar;

  fn to_virtual(self, vp: &Viewport) -> Self::Output {
    VirtualScalar(self.0 * vp.virtual_to_screen_scale())
  }
}

impl ToScreen for VirtualScalar {
  type Output = ScreenScalar;

  fn to_screen(self, vp: &Viewport) -> Self::Output {
    ScreenScalar(self.0 / vp.virtual_to_screen_scale())
  }
}

impl ToVirtual for ScreenPosition {
  type Output = VirtualPosition;

  fn to_virtual(self, vp: &Viewport) -> Self::Output {
    let Vector2 { x: s_x, y: s_y } = self.0;
    let v_x = (s_x - vp.half_screen_width()) / vp.screen_width() * vp.virtual_width() + vp.virtual_center.x;
    let v_y = (vp.half_screen_height() - s_y) / vp.screen_height() * vp.virtual_height() + vp.virtual_center.y;
    VirtualPosition(vec2![v_x, v_y])
  }
}

impl ToScreen for VirtualPosition {
  type Output = ScreenPosition;

  fn to_screen(self, vp: &Viewport) -> Self::Output {
    let Vector2 { x: v_x, y: v_y } = self.0;
    let s_x = (v_x - vp.virtual_center.x + vp.half_virtual_width()) / vp.virtual_width() * vp.screen_width();
    let s_y = (vp.virtual_center.y - v_y + vp.half_virtual_height()) / vp.virtual_height() * vp.screen_height();
    ScreenPosition(vec2![s_x, s_y])
  }
}

impl ToVirtual for ScreenLine {
  type Output = VirtualLine;

  fn to_virtual(self, vp: &Viewport) -> Self::Output {
    let Self { from, to, line_type } = self;
    Self::Output {
      from: from.to_virtual(vp),
      to: to.to_virtual(vp),
      line_type,
    }
  }
}

impl ToScreen for VirtualLine {
  type Output = ScreenLine;

  fn to_screen(self, vp: &Viewport) -> Self::Output {
    let Self { from, to, line_type } = self;
    Self::Output {
      from: from.to_screen(vp),
      to: to.to_screen(vp),
      line_type,
    }
  }
}

impl ToVirtual for ScreenCircle {
  type Output = VirtualCircle;

  fn to_virtual(self, vp: &Viewport) -> Self::Output {
    let Self { center, radius } = self;
    Self::Output {
      center: center.to_virtual(vp),
      radius: radius.to_virtual(vp),
    }
  }
}

impl ToScreen for VirtualCircle {
  type Output = ScreenCircle;

  fn to_screen(self, vp: &Viewport) -> Self::Output {
    let Self { center, radius } = self;
    Self::Output {
      center: center.to_screen(vp),
      radius: radius.to_screen(vp),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_virtual_screen_space_type_conversion() {
    let viewport = Viewport::default();
    let p = vec2![0., 0.];
    let sp = ScreenPosition(p);
    let vp = VirtualPosition(p);
    // let something = vp + sp; // Should not work
    let _vp_add = vp + sp.to_virtual(&viewport); // This should work
  }
}
