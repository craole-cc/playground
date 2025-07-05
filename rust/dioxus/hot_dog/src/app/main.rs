use super::prelude::*;

#[component]
pub fn Main() -> Element {
  rsx! {
    main {
      section { class: "hero-section",

        h2 { style: "font-size: 2.5rem; margin-bottom: 1rem; text-align: center; color: #333;",
          "Featured Gallery"
        }

        p { style: "font-size: 1.1rem; margin-bottom: 2rem; text-align: center; color: #666; max-width: 600px;",
          "Discover amazing images in our interactive carousel gallery. Navigate through our curated collection of stunning visuals."
        }
      
      // ImageCarousel {}
      }
    }
  }
}
