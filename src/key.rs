use cocoa::appkit::NSEventModifierFlags;

pub struct KeyEquivalent<'a> {
  pub key: &'a str,
  pub masks: Option<NSEventModifierFlags>,
}
