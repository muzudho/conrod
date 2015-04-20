
use elmesque::Element;
use position::{Depth, Point};

pub mod button;
pub mod drop_down_list;
pub mod envelope_editor;
pub mod label;
pub mod matrix;
pub mod number_dialer;
pub mod slider;
pub mod text_box;
pub mod toggle;
pub mod xy_pad;

/// A widget element for storage within the Ui's `widget_cache`.
#[derive(Clone, Debug)]
pub struct Widget {
    pub kind: Kind,
    pub xy: Point,
    pub depth: Depth,
    pub element: Option<Element>,
}

impl Widget {

    /// Construct an empty Widget for a vacant widget position within the Ui.
    pub fn empty() -> Widget {
        Widget {
            kind: Kind::NoWidget,
            xy: [0.0, 0.0],
            depth: 0.0,
            element: None,
        }
    }

    /// Construct a Widget from a given kind.
    pub fn new(kind: Kind) -> Widget {
        Widget {
            kind: kind,
            xy: [0.0, 0.0],
            depth: 0.0,
            element: None,
        }
    }

}

/// Algebraic widget type for storing in ui_context
/// and for ease of state-matching.
#[derive(Copy, Clone, Debug)]
pub enum Kind {
    NoWidget,
    Button(button::State),
    DropDownList(drop_down_list::State),
    EnvelopeEditor(envelope_editor::State),
    Label,
    NumberDialer(number_dialer::State),
    Slider(slider::State),
    Spacer,
    TextBox(text_box::State),
    Toggle(toggle::State),
    XYPad(xy_pad::State),
}

impl Kind {
    pub fn matches(&self, other: &Kind) -> bool {
        match (self, other) {
            (&Kind::NoWidget, &Kind::NoWidget) => true,
            (&Kind::Button(_), &Kind::Button(_)) => true,
            (&Kind::DropDownList(_), &Kind::DropDownList(_)) => true,
            (&Kind::EnvelopeEditor(_), &Kind::EnvelopeEditor(_)) => true,
            (&Kind::Label, &Kind::Label) => true,
            (&Kind::NumberDialer(_), &Kind::NumberDialer(_)) => true,
            (&Kind::Slider(_), &Kind::Slider(_)) => true,
            (&Kind::Spacer, &Kind::Spacer) => true,
            (&Kind::TextBox(_), &Kind::TextBox(_)) => true,
            (&Kind::Toggle(_), &Kind::Toggle(_)) => true,
            (&Kind::XYPad(_), &Kind::XYPad(_)) => true,
            _ => false
        }
    }
}

