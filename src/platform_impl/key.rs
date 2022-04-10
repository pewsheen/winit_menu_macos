use cocoa::appkit::NSEventModifierFlags;

#[derive(Debug, Clone)]
pub struct KeyEquivalent<'a> {
  pub key: &'a str,
  pub masks: Option<NSEventModifierFlags>,
}
