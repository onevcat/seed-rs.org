use crate::{
    asset_path,
    generated::css_classes::C,
    image_src, Model, Msg, Page, ScrollHistory, Route,
    Visibility::{self, *},
};
use seed::{prelude::*, *};

//fn header_visibility(
//    menu_visibility: Visibility,
//    scroll_history: &ScrollHistory,
//) -> Visibility {
//    let menu_is_visible = menu_visibility == Visible;
//    // You can go higher on the mobile phones.
//    let at_the_top_or_higher = *scroll_history.back().unwrap_or(&0) <= 0;
//    let scrolling_up = scroll_history.front() >= scroll_history.back();
//
//    if menu_is_visible || at_the_top_or_higher || scrolling_up {
//        Visible
//    } else {
//        Hidden
//    }
//}

pub fn view(model: &Model) -> impl View<Msg> {
//    let show_header =
//        header_visibility(model.menu_visibility, &model.scroll_history)
//            == Visible;

    nav![
        id!("header"),
        class![
            C.fixed,
            C.w_full,
            C.z_10,
            C.top_0,
            C.bg_white,
            C.border_b,
            C.border_gray_400,
        ],
        div![
            class![
                C.w_full,
                C.container,
                C.mx_auto,
                C.flex,
                C.flex_wrap,
                C.items_center,
                C.justify_between,
                C.mt_0,
                C.py_4,
            ],
            view_guide_list_toggle().els(),
            view_logo().els(),
            view_menu_toggle().els(),
            view_menu_content(model.menu_visibility).els(),
        ]
    ]
}

fn view_logo() -> impl View<Msg> {
    div![
        class![
            C.flex,
            C.items_center,
            // lg__
            C.lg__pl_4,
        ],
        a![
            class![
                C.text_gray_900,
                C.text_base,
                C.font_extrabold,
                C.text_xl,
            ],
            attrs!{
                At::Href => Route::Root.to_string()
            },
            "Seed"
        ]
    ]
}

fn view_guide_list_toggle() -> impl View<Msg> {
    div![
        class![
            C.pl_4,
            C.flex,
            // lg__
            C.lg__hidden
        ],
        button![
            id!("view_guide_list_toggle"),
            class![
                C.flex,
                C.items_center,
                C.px_3,
                C.py_2,
                C.border,
                C.rounded,
                C.text_gray_500,
                C.border_gray_600,
                C.hover__text_gray_900,
                C.hover__border_purple_500,
                C.appearance_none,
                C.focus__outline_none,
            ],
            simple_ev(Ev::Click, Msg::ScrollToTop),
            simple_ev(Ev::Click, Msg::ToggleGuideList),
            "Guides",
        ]
    ]
}

fn view_menu_toggle() -> impl View<Msg> {
    div![
        class![
            C.pr_4,
            C.flex,
            // lg__
            C.lg__hidden
        ],
        button![
            id!("menu_toggle"),
            class![
                C.flex,
                C.items_center,
                C.px_3,
                C.py_2,
                C.border,
                C.rounded,
                C.text_gray_500,
                C.border_gray_600,
                C.hover__text_gray_900,
                C.hover__border_purple_500,
                C.appearance_none,
                C.focus__outline_none,
            ],
            simple_ev(Ev::Click, Msg::ToggleMenu),
            "Menu",
        ]
    ]
}

fn view_menu_content(menu_visibility: Visibility) -> impl View<Msg> {
    div![
        id!("menu_content"),
        class![
            C.w_full,
            C.flex_grow,
            C.hidden => menu_visibility == Hidden,
            C.mt_2,
            C.z_20,
            // lg__
            C.lg__flex,
            C.lg__content_center,
            C.lg__items_center,
            C.lg__w_auto,
            C.lg__mt_0,
        ],
        view_search().els(),
        view_links().els(),
    ]
}

fn view_search() -> impl View<Msg> {
    div![
        class![
            C.flex_1,
            C.w_full,
            C.mx_auto,
            C.max_w_sm,
            C.content_center,
            C.py_4,
            // lg__
            C.lg__py_0,
        ],
        div![
            class![
                C.relative,
                C.pl_4,
                C.pr_4,
                // md__
                C.md__pr_0,
            ],
            input![
                class![
                    C.w_full,
                    C.bg_gray_100,
                    C.text_sm,
                    C.text_gray_800,
                    C.placeholder_gray_800,
                    C.border,
                    C.focus__outline_none,
                    C.focus__border_purple_500,
                    C.rounded,
                    C.py_1,
                    C.px_2,
                    C.pl_10,
                    C.appearance_none,
                    C.leading_normal,
                ],
                attrs!{
                    At::Type => "search",
                    At::Placeholder => "Search",
                }
            ],
            div![
                class![
                    C.absolute,
                ],
                style!{
                    St::Top => rem(0.375),
                    St::Left => rem(1.75),
                },
                svg![
                    class![
                        C.fill_current,
                        C.pointer_events_none,
                        C.text_gray_800,
                        C.w_4,
                        C.h_4,
                    ],
                    attrs!{
                        At::ViewBox => "0 0 20 20",
                    },
                    path![
                        attrs!{
                            At::D => "M12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12zM12.9 14.32a8 8 0 1 1 1.41-1.41l5.35 5.33-1.42 1.42-5.33-5.34zM8 14A6 6 0 1 0 8 2a6 6 0 0 0 0 12z",
                        }
                    ]
                ],
            ]
        ]
    ]
}

fn view_links() -> impl View<Msg> {
    ul![
        class![
            C.justify_end,
            C.items_center,
            // lg__
            C.lg__flex,
        ],
        view_link("Active", true).els(),
        view_link("link", false).els(),
        view_link("link", false).els(),
    ]
}

fn view_link(title: &str, active: bool) -> impl View<Msg> {
    li![
        class![
            C.mr_3,
            C.py_2,
            // lg__
            C.lg__py_0,
        ],
        a![
            class![
                C.inline_block,
                C.py_2,
                C.px_4,
                if active { C.text_gray_900 } else { C.text_gray_600 },
                C.hover__text_gray_900 => !active,
                C.hover__underline => !active,
                C.font_bold,
                C.focus__outline_none,
            ],
            attrs!{
                At::Href => "",
            },
            title
        ]
    ]
}