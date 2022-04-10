use crate::platform_impl::key::KeyEquivalent;
use objc::runtime::Sel;

#[derive(Debug, Clone)]
pub struct MenuItemAttributes<'a> {
  pub title: &'a str,
  pub selector: Option<Sel>,
  pub key_equivalent: Option<KeyEquivalent<'a>>,
  /// Initial enabled state. Default to true
  pub enabled: bool,
  /// Initial selected state. Default to false
  pub selected: bool,
}

impl<'a> MenuItemAttributes<'a> {
  pub fn new(title: &'a str) -> Self {
    Self {
      title,
      selector: None,
      key_equivalent: None,
      enabled: true,
      selected: false,
    }
  }
  pub fn with_selector(mut self, selector: Sel) -> Self {
    self.selector = Some(selector);
    self
  }
  pub fn with_key_equivalent(mut self, key_equivalent: KeyEquivalent<'a>) -> Self {
    self.key_equivalent = Some(key_equivalent);
    self
  }
  pub fn with_enabled(mut self, enabled: bool) -> Self {
    self.enabled = enabled;
    self
  }
  pub fn with_selected(mut self, selected: bool) -> Self {
    self.selected = selected;
    self
  }
}
