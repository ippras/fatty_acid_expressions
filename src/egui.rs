use crate::r#const::{
    EXPRESSION, RATIO, SUM,
    ratio::{BIODIESEL, METABOLIC, NUTRITIONAL},
};
use const_format::formatcp;
use egui::{Response, RichText, Ui, Widget};
use egui_l10n::prelude::*;
use egui_phosphor::regular::SIGMA;
use typed_builder::TypedBuilder;
use widgets::buttons::SumButton;

/// Expressions menu button widget
#[derive(Debug, TypedBuilder)]
pub struct ExpressionsMenuButton<'a> {
    #[builder(default, setter(strip_option))]
    sum: Option<&'a mut bool>,
    #[builder(default, setter(strip_option))]
    biodiesel_ratio: Option<&'a mut bool>,
    #[builder(default, setter(strip_option))]
    metabolic_ratio: Option<&'a mut bool>,
    #[builder(default, setter(strip_option))]
    nutritional_ratio: Option<&'a mut bool>,
}

impl Widget for ExpressionsMenuButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.menu_button(
            (
                RichText::new(SIGMA).heading(),
                RichText::new(ui.localize(formatcp!("{EXPRESSION}?PluralCategory=other")))
                    .heading(),
            ),
            |ui| {
                if let Some(selected) = self.sum {
                    SumButton::builder()
                        .selected(selected)
                        .atom(SUM)
                        .hover(formatcp!("{SUM}.hover"))
                        .build()
                        .ui(ui);
                }
                ui.menu_button(
                    (
                        RichText::new(SIGMA).heading(),
                        RichText::new(ui.localize(RATIO)).heading(),
                    ),
                    |ui| {
                        if let Some(selected) = self.biodiesel_ratio {
                            SumButton::builder()
                                .selected(selected)
                                .atom(BIODIESEL)
                                .hover(formatcp!("{BIODIESEL}.hover"))
                                .build()
                                .ui(ui);
                        }
                        if let Some(selected) = self.biodiesel_ratio {
                            SumButton::builder()
                                .selected(selected)
                                .atom(METABOLIC)
                                .hover(formatcp!("{METABOLIC}.hover"))
                                .build()
                                .ui(ui);
                        }
                        if let Some(selected) = self.nutritional_ratio {
                            SumButton::builder()
                                .selected(selected)
                                .atom(NUTRITIONAL)
                                .hover(formatcp!("{NUTRITIONAL}.hover"))
                                .build()
                                .ui(ui);
                        }
                    },
                )
                .response
                .on_hover_text(formatcp!("{RATIO}.hover"));
            },
        )
        .response
        .on_hover_text(formatcp!("{RATIO}.hover?PluralCategory=other"))
        // let mut atoms = (RichText::new(SIGMA), RichText::new(ui.localize(self.atom)));
        // atoms = if let Some(size) = self.size {
        //     (atoms.0.size(size), atoms.1.size(size))
        // } else {
        //     (atoms.0.heading(), atoms.1.heading())
        // };
        // let mut response = ui.toggle_value(self.selected, atoms);
        // if let Some(hover) = self.hover {
        //     response = response.on_hover_ui(|ui| {
        //         ui.label(ui.localize(hover));
        //     })
        // }
    }
}

pub struct Ratio<'a> {
    biodiesel: &'a mut bool,
    metabolic: &'a mut bool,
    nutritional: &'a mut bool,
}

// ui.menu_button(
//     (
//         RichText::new(SIGMA).heading(),
//         RichText::new(ui.localize(formatcp!("{PROPERTY}?PluralCategory=other")))
//             .heading(),
//     ),
//     |ui| {
//         SumButton::builder()
//             .selected(&mut state.windows.open_expressions_sum)
//             .atom(SUM)
//             .hover(formatcp!("{SUM}.hover"))
//             .build()
//             .ui(ui);
//         ui.menu_button(
//             (
//                 RichText::new(SIGMA).heading(),
//                 RichText::new(ui.localize(RATIO)).heading(),
//             ),
//             |ui| {
//                 SumButton::builder()
//                     .selected(&mut state.windows.open_expressions_ratio_biodiesel)
//                     .atom(BIODIESEL)
//                     .hover(formatcp!("{BIODIESEL}.hover"))
//                     .build()
//                     .ui(ui);
//                 SumButton::builder()
//                     .selected(&mut state.windows.open_expressions_ratio_metabolic)
//                     .atom(METABOLIC)
//                     .hover(formatcp!("{METABOLIC}.hover"))
//                     .build()
//                     .ui(ui);
//                 SumButton::builder()
//                     .selected(&mut state.windows.open_expressions_ratio_nutritional)
//                     .atom(NUTRITIONAL)
//                     .hover(formatcp!("{NUTRITIONAL}.hover"))
//                     .build()
//                     .ui(ui);
//             },
//         );
//     },
// );
