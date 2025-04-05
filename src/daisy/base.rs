pub const BTN: &str = "btn";

pub const PRIMARY: &str = "primary";
pub const ACCENT: &str = "accent";
pub const SECONDARY: &str = "secondary";
pub const SUCCESS: &str = "success";
pub const INFO: &str = "info";
pub const WARNING: &str = "warning";
pub const ERROR: &str = "error";
pub const OUTLINE: &str = "outline";
pub const GHOST: &str = "ghost";
pub const LINK: &str = "link";
pub const SOFT: &str = "soft";
pub const ACTIVE: &str = "active";
pub const DISABLED: &str = "disabled";
pub const XS: &str = "xs";
pub const SM: &str = "sm";
pub const MD: &str = "md";
pub const LG: &str = "lg";
pub const XL: &str = "xl";
pub const WIDE: &str = "wide";
pub const BLOCK: &str = "block";
pub const SQUARE: &str = "square";
pub const CIRCLE: &str = "circle";

pub fn combine(first: &str, second: &str) -> String {
    format!("btn {}-{}", first, second)
}
