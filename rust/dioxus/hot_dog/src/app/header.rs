use super::prelude::*;

#[component]
pub fn Header() -> Element {
  rsx! {
    header {
      img { src: LOGO }
      h1 { style: "color: white; margin: 0; font-size: 1.5rem;", {format!("{TITLE}!")} }
      nav {
        ul {
          li {
            a {
              href: "#",
              style: "color: white; text-decoration: none; padding: 0.5rem 1rem; border-radius: 4px; transition: background-color 0.3s;",
              "Home"
            }
          }
          li {
            a {
              href: "#",
              style: "color: white; text-decoration: none; padding: 0.5rem 1rem; border-radius: 4px; transition: background-color 0.3s;",
              "Gallery"
            }
          }
          li {
            a {
              href: "#",
              style: "color: white; text-decoration: none; padding: 0.5rem 1rem; border-radius: 4px; transition: background-color 0.3s;",
              "About"
            }
          }
          li {
            a {
              href: "#",
              style: "color: white; text-decoration: none; padding: 0.5rem 1rem; border-radius: 4px; transition: background-color 0.3s;",
              "Contact"
            }
          }
        }
      }
    }
  }
}
