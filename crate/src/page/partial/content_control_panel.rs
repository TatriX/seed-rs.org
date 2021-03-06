use crate::{
    generated::css_classes::C, page::partial::image, Guide, Mode, Model, Msg,
    Route,
};
use seed::{prelude::*, *};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Position {
    Top,
    Bottom,
}

pub fn view(
    selected_guide: &Guide,
    position: Position,
    model: &Model,
) -> impl View<Msg> {
    div![
        class![
            if position == Position::Top {
                C.mb_8
            } else {
                C.mt_8
            },
            C.w_full,
            C.flex,
            C.justify_between,
            C.text_green_500,
            C.text_sm,
            // lg__
            C.lg__ml_auto,
        ],
        // previous guide
        previous_guide(selected_guide, &model.guides).map_or_else(
            || view_empty_column().els(),
            |previous_guide| view_previous_guide_link(previous_guide).els()
        ),
        // mode toggle or edit this page button
        if position == Position::Top {
            view_mode_toggle(model.in_prerendering, model.mode).els()
        } else {
            view_edit_this_page(selected_guide.edit_url).els()
        },
        // next guide
        next_guide(selected_guide, &model.guides).map_or_else(
            || view_empty_column().els(),
            |next_guide| view_next_guide_link(next_guide).els()
        ),
    ]
}

// ------ view empty column ------

fn view_empty_column() -> impl View<Msg> {
    div![class![C.flex_1,]]
}

// ------ view mode toggle ------

fn view_mode_toggle(in_prerendering: bool, mode: Mode) -> impl View<Msg> {
    div![
        class![C.flex_1, C.flex, C.justify_center,],
        div![
            class![
                C.flex,
                C.items_center,
                C.px_3,
                C.text_gray_500,
                C.border,
                C.border_gray_400,
                C.cursor_pointer,
                C.rounded_full,
                C.hover__underline,
                C.hover__text_gray_700,
                C.hover__border_gray_600,
            ],
            simple_ev(Ev::Click, Msg::ToggleMode),
            span![
                class![C.whitespace_no_wrap, C.flex, C.items_center,],
                if in_prerendering {
                    div![
                        class![C.mr_1, C.h_4, C.w_4, C.rotate,],
                        image::spinner_svg().els()
                    ]
                } else {
                    empty![]
                },
                span![format!(
                    "{} mode",
                    match mode {
                        Mode::Light => "Dark",
                        Mode::Dark => "Light",
                    }
                ),]
            ]
        ]
    ]
}

// ------ view edit this page & feedback ------

fn view_edit_this_page(edit_url: &str) -> impl View<Msg> {
    div![
        class![C.flex_1, C.flex, C.justify_center,],
        a![
            class![
                C.flex,
                C.items_center,
                C.text_blue_500,
                C.whitespace_no_wrap,
                C.hover__underline,
                C.hover__text_blue_700,
            ],
            attrs! {
                At::Href => edit_url,
            },
            span!["Edit this page",]
        ],
        span![class![C.flex, C.mx_1, C.items_center,], "|"],
        a![
            class![
                C.flex,
                C.items_center,
                C.text_blue_500,
                C.whitespace_no_wrap,
                C.hover__underline,
                C.hover__text_blue_700,
            ],
            attrs! {
                At::Href => "https://github.com/seed-rs/seed/issues/303",
            },
            span!["Feedback",]
        ]
    ]
}

// ------ view previous & next guide link ------

fn view_previous_guide_link(previous_guide: &Guide) -> impl View<Msg> {
    div![
        class![C.flex_1, C.flex, C.justify_start,],
        a![
            class![
                C.flex,
                C.hover__underline,
                C.hover__text_green_700,
                C.focus__outline_none,
            ],
            attrs! {
                At::Href => Route::Guide(previous_guide.slug.to_owned()).to_string(),
            },
            view_previous_icon().els(),
            div![
                class![
                    C.font_bold,
                    C.m_auto,
                    C.pb_1,
                    C.hidden,
                    // sm__,
                    C.sm__block,
                ],
                previous_guide.menu_title,
            ],
        ]
    ]
}

fn view_next_guide_link(next_guide: &Guide) -> impl View<Msg> {
    div![
        class![C.flex_1, C.flex, C.justify_end,],
        a![
            class![
                C.flex,
                C.hover__underline,
                C.hover__text_green_700,
                C.focus__outline_none,
            ],
            attrs! {
                At::Href => Route::Guide(next_guide.slug.to_owned()).to_string(),
            },
            div![
                class![
                    C.font_bold,
                    C.m_auto,
                    C.pb_1,
                    C.hidden,
                    // sm__,
                    C.sm__block,
                ],
                next_guide.menu_title,
            ],
            view_next_icon().els(),
        ]
    ]
}

// ------ view previous & next icon ------

fn view_previous_icon() -> impl View<Msg> {
    div![
        class![C.h_8, C.w_8,],
        style! {
            St::Transform => "rotate(180deg)",
        },
        image::next_icon_svg().els()
    ]
}

fn view_next_icon() -> impl View<Msg> {
    div![class![C.h_8, C.w_8], image::next_icon_svg().els()]
}

// ------ get previous & next guide ------

pub fn previous_guide<'a>(
    selected_guide: &Guide,
    guides: &'a [Guide],
) -> Option<&'a Guide> {
    let selected_guide_index =
        guides.iter().position(|guide| guide == selected_guide)?;

    selected_guide_index.checked_sub(1).and_then(|index| guides.get(index))
}

pub fn next_guide<'a>(
    selected_guide: &Guide,
    guides: &'a [Guide],
) -> Option<&'a Guide> {
    let selected_guide_index =
        guides.iter().position(|guide| guide == selected_guide)?;

    selected_guide_index.checked_add(1).and_then(|index| guides.get(index))
}
