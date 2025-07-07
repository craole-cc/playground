use super::prelude::*;

#[component]
pub fn Footer() -> Element {
  rsx! {
    footer {
      div {
        class: "footer-content",
        style: "display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem;",

        div { style: "color: #666;", "© 2024 My Gallery Site. All rights reserved." }

        div { style: "display: flex; gap: 1rem;",

          a {
            href: "#",
            style: "color: #117eeb; text-decoration: none;",
            "Privacy Policy"
          }

          a {
            href: "#",
            style: "color: #117eeb; text-decoration: none;",
            "Terms of Service"
          }

          a {
            href: "#",
            style: "color: #117eeb; text-decoration: none;",
            "Contact Us"
          }
        }
      }
    }
  }
}
