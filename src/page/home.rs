use crate::{
    generated::css_classes::C, image_src, Msg, Urls, MAIL_TO_HELLWEB,
    MAIL_TO,
};
use seed::{prelude::*, *};

#[allow(clippy::too_many_lines)]
pub fn view(base_url: &Url) -> Node<Msg> {
    div![
        C![
            C.flex_grow,
        ],
        // Main section
        section![
            C![
                C.relative,
                C.h_690px,
                C.bg_gray_1,
                // sm__
                C.sm__h_890px,
                // lg__
                C.lg__h_1420px,
            ],
            // Left background
            div![
                C![
                    C.absolute,
                    C.left_0,
                    C.inset_y_0,
                    C.w_1of2,
                    C.bg_yellow_4,
                ]
            ],
            div![
                C![
                    C.relative,
                    C.flex,
                    C.justify_center,
                ],
                // Martin Kavík container
                div![
                    C![
                        C.h_360px,
                        C.rounded_bl_90px,
                        C.bg_gray_1,
                        // sm__
                        C.sm__h_550px,
                        // lg__
                        C.lg__h_860px,
                        C.lg__rounded_bl_260px,
                    ],
                    // Martin Kavík
                    div![
                        C![
                            C.mt_40,
                            C.ml_12,
                            C.w_xs,
                            C.font_display,
                            // sm__
                            C.sm__mt_64,
                            C.sm__ml_20,
                            C.sm__w_md,
                            // lg__
                            C.lg__mt_72,
                            C.lg__ml_20,
                            C.lg__w_216,
                        ],
                        h1![
                            C![
                                C.inline,
                                C.leading_tight,
                                C.text_31,
                                C.text_gray_10
                                // sm__
                                C.sm__text_40,
                                // lg__
                                C.lg__leading_none,
                                C.lg__text_120,
                            ],
                            span![
                                "Don "
                            ],
                            span![
                                C![
                                    C.font_bold
                                ],
                                "Dall"
                            ],
                        ],
                        span![
                            C![
                                C.text_21,
                                C.text_gray_7,
                                // sm__
                                C.sm__text_32,
                                // lg__
                                C.lg__text_60,
                            ],
                            "\u{00A0}is",
                            br![],
                            "a software engineer",
                            br![],
                            "with 2+ years of experience",
                            br![],
                            "who likes to write code and ..."
                        ]
                    ],
                ],
            ],
            // Gear
            img![
                C![
                    C.absolute
                    C.top_0,
                    C.left_full,
                    C._ml_40,
                    C._mt_56,
                    C.w_md,
                    C.max_w_none,
                    C.blur,
                    // sm__
                    C.sm___mt_64,
                    C.sm__w_750px,
                    // lg__
                    C.lg__w_900px,
                ],
                attrs!{
                    At::Src => image_src("gear.svg")
                }
            ],
        ],
        // Seed section
        section![
            C![
                C.relative,
                C._mt_48,
                C.pt_px,
                C.rounded_tl_140px,
                C.bg_blue_8,
                // lg__
                C.lg___mt_80,
                C.lg__rounded_tl_330px,
            ],
            // Main list
            div![
                C![
                    C._ml_5,
                    C._mt_48,
                    C.max_w_lg,
                    C.flex,
                    C.justify_end,
                    C.relative,
                    // sm__
                    C.max_w_4xl,
                    // lg__
                    C.lg___mt_92,
                    C.lg__max_w_8xl,
                    C.lg__mx_auto,
                ],
                // Right background
                div![
                    C![
                        C.absolute,
                        C.right_0,
                        C.inset_y_0,
                        C._right_50vw,
                        C.w_50vw,
                        C.bg_gray_1
                    ]
                ],
                // List
                div![
                    C![
                        C.relative,
                        C.pl_4,
                        C.rounded_bl_45px,
                        C.font_display,
                        C.text_17,
                        C.text_gray_8,
                        C.bg_gray_1,
                        C.overflow_hidden,
                        // sm__
                        C.sm__pl_8,
                        C.sm__pr_2,
                        C.sm__text_26,
                        // lg__
                        C.lg__pl_16,
                        C.lg__text_45,
                        C.lg__rounded_bl_140px,
                    ],
                    ul![
                        C![
                            C.w_xs,
                            C.pl_2,
                            C.py_8,
                            // sm__
                            C.sm__w_120,
                            // lg__
                            C.lg__w_204,
                            C.lg__pt_32,
                            C.lg__pb_24,
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    // https://stackoverflow.com/a/39900080
                                    "▶\u{fe0e}"
                                ],
                                "To solve real work problems."
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "Cool solutions."
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "Rust, just Rust."
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    C![
                                        C.text_blue_6,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    "Receiving mails. ",
                                    a![
                                        attrs!{
                                            At::Href => MAIL_TO
                                        },
                                        C![
                                            C.underline,
                                            C.underline_yellow_7,
                                            C.font_semibold
                                        ],
                                        "awesomealpineibex@gmail.com"
                                    ]
                                ]
                            ]
                        ],
                    ]
                ]
            ],
            div![
                C![
                    C.flex,
                    C.flex_col,
                    C.items_center
                ],
                // Section content container
                div![
                    C![
                        C.mt_20,
                        C.w_xs,
                        C.px_2,
                        // sm__
                        C.sm__mt_48,
                        C.sm__w_md,
                        // lg__
                        C.lg__mt_64,
                        C.lg__w_236,
                    ],
                    // Github projects
                    h2![
                        C![
                            C.font_display,
                            C.text_23,
                            C.text_blue_3,
                            C.text_center,
                            // sm__
                            C.sm__text_50,
                            // lg__
                            C.lg__text_80,
                        ],
                        span![
                            C![
                                C.font_thin
                            ],
                            "TOP-5"
                        ],
                        span![
                            C![
                                C.font_normal
                            ],
                            " GITHUB PROJECTS"
                        ]
                    ],
                    // Tasky
                    a![
                        attrs!{
                            At::Href => "https://github.com/tasky-aws"
                        },
                        C![
                            C.block,
                            C.relative,
                            C.mt_8,
                            C.pt_5,
                            C.pb_3,
                            C.w_36,
                            C.rounded_tr_28px,
                            C.bg_blue_2,
                            C.shadow_glow,
                            // sm__
                            C.sm__mt_24,
                            C.sm__pt_8,
                            C.sm__pb_8,
                            C.sm__w_64,
                            C.sm__rounded_tr_55px,
                            // lg__
                            C.lg__mt_32,
                            C.lg__pt_12,
                            C.lg__pb_12,
                            C.lg__w_md,
                            C.lg__rounded_tr_90px,
                        ],
                        // Extended background
                        div![
                            C![
                                C.absolute,
                                C.left_0,
                                C.inset_y_0,
                                C._left_50vw,
                                C.w_50vw,
                                C.bg_blue_2,
                                C.shadow_glow,
                            ]
                        ],
                        // Logo
                        img![
                            C![
                                C.h_18,
                                C.max_w_none,
                                // sm__
                                C.sm__h_32
                                // lg__
                                C.lg__ml_32,
                                C.lg__h_40,
                            ],
                            attrs!{
                                At::Src => image_src("tasky_logo.svg")
                            }
                        ]
                    ],
                    ul![
                        C![
                            C.mt_10,
                            C.text_blue_1,
                            // sm__
                            C.sm__mt_20,
                            // lg__
                            C.lg__mt_32,
                        ],
                        li![
                            C![
                                C.my_3,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                div![
                                    a![
                                        attrs!{
                                            At::Href => "https://github.com/MartinKavik/awesome-seed-rs"
                                        },
                                        h3![
                                            C![
                                                C.inline,
                                                C.text_18,
                                                C.font_bold,
                                                // sm__
                                                C.sm__text_26,
                                                // lg__
                                                C.lg__text_45,
                                            ],
                                            "Tasky"
                                        ],
                                    ],
                                    "\u{00A0}is an open-source local AWS client providing read only access to ECS and logging which will enable quicker observability."
                                ]
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                                // sm__
                                C.sm__mt_8,
                                // lg__
                                C.lg__mt_16,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "I'm the main contributor."
                            ]
                        ],
                        li![
                            C![
                                C.my_3,
                                // sm__
                                C.sm__mt_8,
                                // lg__
                                C.lg__mt_16,
                            ],
                            div![
                                C![
                                    C.flex,
                                    C.flex_no_wrap,
                                ],
                                div![
                                    C![
                                        C.text_yellow_4,
                                        C.mr_2,
                                        // lg__
                                        C.lg__mr_4,
                                    ],
                                    "▶\u{fe0e}"
                                ],
                                "I've designed the logo."
                            ]
                        ],
                    ],
                    a![
                        attrs!{
                            At::Href => "https://github.com/MartinKavik/awesome-seed-rs"
                        },
                        C![
                            C.block,
                            C.mt_10,
                            C.mb_24,
                            C.mr_2,
                            C.text_right,
                            C.font_display,
                            // sm__
                            C.sm__mt_16,
                            C.sm__mb_48,
                            // lg__
                            C.lg__mt_24,
                            C.lg__mb_64,
                        ],
                        span![
                            C![
                                C.text_blue_4
                            ],
                            "MartinKavik/"
                        ],
                        span![
                            C![
                                C.text_blue_2
                            ],
                            "awesome-seed-rs"
                        ],
                        img![
                            C![
                                C.inline
                                C.mb_4,
                                C.ml_px,
                                C.w_3,
                                // sm__
                                C.sm__mb_6,
                                C.sm__w_4,
                                // lg__
                                C.lg__mb_8,
                                C.lg__w_5,
                            ],
                            attrs!{
                                At::Src => image_src("link_arrow.svg")
                            }
                        ]
                    ]
                ]
            ]
        ],
        // Circles
        div![
            C![
                C.absolute,
                C.left_1of2,
                C.top_0,
                C.mt_310px,
                C.ml_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__ml_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
                // lg__
                C.lg__mt_1290px,
                C.lg__ml_108,
            ]
        ],
        div![
            C![
                C.absolute,
                C.right_1of2,
                C.top_0,
                C.mt_790px,
                C.mr_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_1310px,
                C.sm__mr_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
                // lg__
                C.lg__mt_2840px,
                C.lg__mr_108,
            ]
        ],
        div![
            C![
                C.absolute,
                C.left_1of2,
                C.top_0,
                C.mt_1760px,
                C.ml_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_3040px,
                C.sm__ml_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
                // lg__
                C.lg__mt_5030px,
                C.lg__ml_108,
            ]
        ],
        div![
            C![
                C.absolute,
                C.right_1of2,
                C.top_0,
                C.mt_2340px,
                C.mr_38,
                C.w_1240px,
                C.h_1240px,
                C.rounded_full,
                C.border_blue_2,
                C.border_2,
                C.opacity_10,
                // sm__
                C.sm__mt_3870px,
                C.sm__mr_64,
                C.sm__h_2560px,
                C.sm__w_2560px,
                // lg__
                C.lg__mt_6070px,
                C.lg__mr_108,
            ]
        ],
    ]
}
