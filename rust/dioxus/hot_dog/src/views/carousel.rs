use super::prelude::*;

#[derive(Clone, PartialEq)]
pub struct CarouselItem {
  pub id: usize,
  pub title: String,
  pub image_url: String,
  pub description: String
}

#[component]
pub fn ImageCarousel() -> Element {
  let mut current_index = use_signal(|| 0usize);

  // Dummy data for the carousel
  let carousel_items = use_memo(|| {
    vec![
      CarouselItem {
        id: 1,
        title: "Mountain Landscape".to_string(),
        image_url: "https://picsum.photos/800/400?random=1".to_string(),
        description: "Beautiful mountain scenery with snow-capped peaks"
          .to_string()
      },
      CarouselItem {
        id: 2,
        title: "Ocean Sunset".to_string(),
        image_url: "https://picsum.photos/800/400?random=2".to_string(),
        description: "Stunning ocean sunset with vibrant colors".to_string()
      },
      CarouselItem {
        id: 3,
        title: "Forest Path".to_string(),
        image_url: "https://picsum.photos/800/400?random=3".to_string(),
        description: "Peaceful forest path surrounded by tall trees"
          .to_string()
      },
      CarouselItem {
        id: 4,
        title: "City Skyline".to_string(),
        image_url: "https://picsum.photos/800/400?random=4".to_string(),
        description: "Modern city skyline during golden hour".to_string()
      },
      CarouselItem {
        id: 5,
        title: "Desert Dunes".to_string(),
        image_url: "https://picsum.photos/800/400?random=5".to_string(),
        description: "Rolling sand dunes in the desert".to_string()
      },
    ]
  });

  let items = carousel_items();
  let total_items = items.len();

  let next_slide = move |_: Event<MouseData>| {
    current_index.set((current_index() + 1) % total_items);
  };

  let prev_slide = move |_: Event<MouseData>| {
    current_index.set(if current_index() == 0 {
      total_items - 1
    } else {
      current_index() - 1
    });
  };

  let go_to_slide = move |index: usize| {
    move |_: Event<MouseData>| {
      current_index.set(index);
    }
  };

  let current_item = &items[current_index()];

  rsx! {
    div {
      class: "carousel-container",
      style: "
            position: relative;
            max-width: 800px;
            margin: 0 auto;
            border-radius: 12px;
            overflow: hidden;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
            background: white;
        ",

      // Main carousel display
      div {
        class: "carousel-main",
        style: "position: relative; height: 400px; overflow: hidden;",
        img {
          src: "{current_item.image_url}",
          alt: "{current_item.title}",
          style: "
                width: 100%;
                height: 100%;
                object-fit: cover;
                transition: transform 0.3s ease;
            ",
        }
      }
    }
  }
  // rsx! {
  //     div {
  //         class: "carousel-container",
  //         style: "
  //               position: relative;
  //               max-width: 800px;
  //               margin: 0 auto;
  //               border-radius: 12px;
  //               overflow: hidden;
  //               box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  //               background: white;
  //           ",

  //         // Main carousel display
  //         div {
  //             class: "carousel-main",
  //             style: "position: relative; height: 400px; overflow: hidden;",

  //             img {
  //                 src: "{current_item.image_url}",
  //                 alt: "{current_item.title}",
  //                 style: "
  //                       width: 100%;
  //                       height: 100%;
  //                       object-fit: cover;
  //                       transition: transform 0.3s ease;
  //                   "
  //             }

  //             // Navigation buttons
  //             button {
  //                 class: "carousel-btn prev",
  //                 style: "
  //                       position: absolute;
  //                       left: 10px;
  //                       top: 50%;
  //                       transform: translateY(-50%);
  //                       background: rgba(0, 0, 0, 0.5);
  //                       color: white;
  //                       border: none;
  //                       padding: 10px 15px;
  //                       font-size: 18px;
  //                       cursor: pointer;
  //                       border-radius: 50%;
  //                       transition: background-color 0.3s;
  //                   ",
  //                 onclick: prev_slide,
  //                 "‹"
  //             }

  //             button {
  //                 class: "carousel-btn next",
  //                 style: "
  //                       position: absolute;
  //                       right: 10px;
  //                       top: 50%;
  //                       transform: translateY(-50%);
  //                       background: rgba(0, 0, 0, 0.5);
  //                       color: white;
  //                       border: none;
  //                       padding: 10px 15px;
  //                       font-size: 18px;
  //                       cursor: pointer;
  //                       border-radius: 50%;
  //                       transition: background-color 0.3s;
  //                   ",
  //                 onclick: next_slide,
  //                 "›"
  //             }
  //         }

  //         // Image info
  //         div {
  //             class: "carousel-info",
  //             style: "
  //                   padding: 20px;
  //                   background: white;
  //               ",

  //             h3 {
  //                 style: "
  //                       margin: 0 0 10px 0;
  //                       font-size: 1.3rem;
  //                       color: #333;
  //                   ",
  //                 "{current_item.title}"
  //             }

  //             p {
  //                 style: "
  //                       margin: 0;
  //                       color: #666;
  //                       line-height: 1.5;
  //                   ",
  //                 "{current_item.description}"
  //             }
  //         }

  //         // Dot indicators
  //         div {
  //             class: "carousel-dots",
  //             style: "
  //                   display: flex;
  //                   justify-content: center;
  //                   gap: 8px;
  //                   padding: 15px 20px;
  //                   background: white;
  //               ",

  //             for (index, _item) in items.iter().enumerate() {
  //                 button {
  //                     key: "dot-{index}",
  //                     class: "dot",
  //                     style: "
  //                           width: 12px;
  //                           height: 12px;
  //                           border-radius: 50%;
  //                           border: none;
  //                           cursor: pointer;
  //                           transition: background-color 0.3s;
  //                           background-color: {if index == current_index() {
  // \"#117eeb\" } else { \"#ddd\" }};                       ",
  //                     onclick: go_to_slide(index)
  //                 }
  //             }
  //         }
  //     }
  // }
}
