use imgui::Style;
use imnodes::ImNodesStyle;

pub fn set_raikiri_style(style: &mut Style) {
    // My tweaks
    style.popup_rounding = 4.0;
    style.child_rounding = style.popup_rounding;
    style.window_rounding = style.child_rounding;
    style.window_border_size = 1.0;
    style.popup_border_size = 0.5;
    style.child_border_size = style.popup_border_size;

    // imGuiIO.Fonts->AddFontFromFileTTF("../data/Fonts/Ruda-Bold.ttf", 15.0,
    // &config);
    style.scrollbar_rounding = 4.0;
    style.grab_rounding = style.scrollbar_rounding;
    style.frame_rounding = style.grab_rounding;

    style[imgui::StyleColor::Text] = [0.95, 0.96, 0.98, 1.00];
    style[imgui::StyleColor::TextDisabled] = [0.36, 0.42, 0.47, 1.00];
    style[imgui::StyleColor::WindowBg] = [0.11, 0.15, 0.17, 1.00];
    style[imgui::StyleColor::ChildBg] = [0.15, 0.18, 0.22, 1.00];
    style[imgui::StyleColor::PopupBg] = [0.08, 0.08, 0.08, 0.94];
    style[imgui::StyleColor::Border] = [0.08, 0.10, 0.12, 1.00];
    style[imgui::StyleColor::BorderShadow] = [0.1, 0.1, 0.1, 1.00];
    style[imgui::StyleColor::FrameBg] = [0.20, 0.25, 0.29, 1.00];
    style[imgui::StyleColor::FrameBgHovered] = [0.12, 0.20, 0.28, 1.00];
    style[imgui::StyleColor::FrameBgActive] = [0.09, 0.12, 0.14, 1.00];
    style[imgui::StyleColor::TitleBg] = [0.09, 0.12, 0.14, 0.65];
    style[imgui::StyleColor::TitleBgActive] = [0.08, 0.10, 0.12, 1.00];
    style[imgui::StyleColor::TitleBgCollapsed] = [0.00, 0.00, 0.00, 0.51];
    style[imgui::StyleColor::MenuBarBg] = [0.15, 0.18, 0.22, 1.00];
    style[imgui::StyleColor::ScrollbarBg] = [0.02, 0.02, 0.02, 0.39];
    style[imgui::StyleColor::ScrollbarGrab] = [0.20, 0.25, 0.29, 1.00];
    style[imgui::StyleColor::ScrollbarGrabHovered] = [0.18, 0.22, 0.25, 1.00];
    style[imgui::StyleColor::ScrollbarGrabActive] = [0.09, 0.21, 0.31, 1.00];
    style[imgui::StyleColor::CheckMark] = [0.28, 0.56, 1.00, 1.00];
    style[imgui::StyleColor::SliderGrab] = [0.28, 0.56, 1.00, 1.00];
    style[imgui::StyleColor::SliderGrabActive] = [0.37, 0.61, 1.00, 1.00];
    style[imgui::StyleColor::Button] = [0.20, 0.25, 0.29, 1.00];
    style[imgui::StyleColor::ButtonHovered] = [0.28, 0.56, 1.00, 1.00];
    style[imgui::StyleColor::ButtonActive] = [0.06, 0.53, 0.98, 1.00];
    style[imgui::StyleColor::Header] = [0.20, 0.25, 0.29, 0.55];
    style[imgui::StyleColor::HeaderHovered] = [0.26, 0.59, 0.98, 0.80];
    style[imgui::StyleColor::HeaderActive] = [0.26, 0.59, 0.98, 1.00];
    style[imgui::StyleColor::Separator] = [0.20, 0.25, 0.29, 1.00];
    style[imgui::StyleColor::SeparatorHovered] = [0.10, 0.40, 0.75, 0.78];
    style[imgui::StyleColor::SeparatorActive] = [0.10, 0.40, 0.75, 1.00];
    style[imgui::StyleColor::ResizeGrip] = [0.26, 0.59, 0.98, 0.25];
    style[imgui::StyleColor::ResizeGripHovered] = [0.26, 0.59, 0.98, 0.67];
    style[imgui::StyleColor::ResizeGripActive] = [0.26, 0.59, 0.98, 0.95];
    style[imgui::StyleColor::Tab] = [0.11, 0.15, 0.17, 1.00];
    style[imgui::StyleColor::TabHovered] = [0.26, 0.59, 0.98, 0.80];
    style[imgui::StyleColor::TabActive] = [0.20, 0.25, 0.29, 1.00];
    style[imgui::StyleColor::TabUnfocused] = [0.11, 0.15, 0.17, 1.00];
    style[imgui::StyleColor::TabUnfocusedActive] = [0.11, 0.15, 0.17, 1.00];
    style[imgui::StyleColor::PlotLines] = [0.61, 0.61, 0.61, 1.00];
    style[imgui::StyleColor::PlotLinesHovered] = [1.00, 0.43, 0.35, 1.00];
    style[imgui::StyleColor::PlotHistogram] = [0.90, 0.70, 0.00, 1.00];
    style[imgui::StyleColor::PlotHistogramHovered] = [1.00, 0.60, 0.00, 1.00];
    style[imgui::StyleColor::TextSelectedBg] = [0.26, 0.59, 0.98, 0.35];
    style[imgui::StyleColor::DragDropTarget] = [1.00, 1.00, 0.00, 0.90];
    style[imgui::StyleColor::NavHighlight] = [0.26, 0.59, 0.98, 1.00];
    style[imgui::StyleColor::NavWindowingHighlight] = [1.00, 1.00, 1.00, 0.70];
    style[imgui::StyleColor::NavWindowingDimBg] = [0.80, 0.80, 0.80, 0.20];
    style[imgui::StyleColor::ModalWindowDimBg] = [0.80, 0.80, 0.80, 0.35];
}

// Obtained from
// https://www.unknowncheats.me/forum/1547436-post1.html?s=78e3c8907fe6b443b8422ca0c24c10
pub fn set_extasy_style(style: &mut Style) {
    style.window_padding = [15.0, 15.0];
    style.window_rounding = 5.0;
    style.frame_padding = [5.0, 5.0];
    style.frame_rounding = 4.0;
    style.item_spacing = [12.0, 8.0];
    style.item_inner_spacing = [8.0, 6.0];
    style.indent_spacing = 25.0;
    style.scrollbar_size = 15.0;
    style.scrollbar_rounding = 9.0;
    style.grab_min_size = 5.0;
    style.grab_rounding = 3.0;
    style.alpha = 1.0;

    style[imgui::StyleColor::Text] = [0.80, 0.80, 0.83, 1.00];
    style[imgui::StyleColor::TextDisabled] = [0.24, 0.23, 0.29, 1.00];
    style[imgui::StyleColor::WindowBg] = [0.06, 0.05, 0.07, 1.00];
    style[imgui::StyleColor::ChildBg] = [0.07, 0.07, 0.09, 1.00];
    style[imgui::StyleColor::PopupBg] = [0.07, 0.07, 0.09, 1.00];
    style[imgui::StyleColor::Border] = [0.80, 0.80, 0.83, 0.88];
    style[imgui::StyleColor::BorderShadow] = [0.92, 0.91, 0.88, 0.00];
    style[imgui::StyleColor::FrameBg] = [0.10, 0.09, 0.12, 1.00];
    style[imgui::StyleColor::FrameBgHovered] = [0.24, 0.23, 0.29, 1.00];
    style[imgui::StyleColor::FrameBgActive] = [0.56, 0.56, 0.58, 1.00];
    style[imgui::StyleColor::TitleBg] = [0.10, 0.09, 0.12, 1.00];
    style[imgui::StyleColor::TitleBgCollapsed] = [1.00, 0.98, 0.95, 0.75];
    style[imgui::StyleColor::TitleBgActive] = [0.07, 0.07, 0.09, 1.00];
    style[imgui::StyleColor::MenuBarBg] = [0.10, 0.09, 0.12, 1.00];
    style[imgui::StyleColor::ScrollbarBg] = [0.10, 0.09, 0.12, 1.00];
    style[imgui::StyleColor::ScrollbarGrab] = [0.80, 0.80, 0.83, 0.31];
    style[imgui::StyleColor::ScrollbarGrabHovered] = [0.56, 0.56, 0.58, 1.00];
    style[imgui::StyleColor::ScrollbarGrabActive] = [0.06, 0.05, 0.07, 1.00];
    style[imgui::StyleColor::CheckMark] = [0.80, 0.80, 0.83, 0.31];
    style[imgui::StyleColor::SliderGrab] = [0.80, 0.80, 0.83, 0.31];
    style[imgui::StyleColor::SliderGrabActive] = [0.06, 0.05, 0.07, 1.00];
    style[imgui::StyleColor::Button] = [0.10, 0.09, 0.12, 1.00];
    style[imgui::StyleColor::ButtonHovered] = [0.24, 0.23, 0.29, 1.00];
    style[imgui::StyleColor::ButtonActive] = [0.56, 0.56, 0.58, 1.00];
    style[imgui::StyleColor::Header] = [0.10, 0.09, 0.12, 1.00];
    style[imgui::StyleColor::HeaderHovered] = [0.56, 0.56, 0.58, 1.00];
    style[imgui::StyleColor::HeaderActive] = [0.06, 0.05, 0.07, 1.00];
    style[imgui::StyleColor::Separator] = [0.56, 0.56, 0.58, 1.00];
    style[imgui::StyleColor::SeparatorHovered] = [0.24, 0.23, 0.29, 1.00];
    style[imgui::StyleColor::SeparatorActive] = [0.56, 0.56, 0.58, 1.00];
    style[imgui::StyleColor::ResizeGrip] = [0.00, 0.00, 0.00, 0.00];
    style[imgui::StyleColor::ResizeGripHovered] = [0.56, 0.56, 0.58, 1.00];
    style[imgui::StyleColor::ResizeGripActive] = [0.06, 0.05, 0.07, 1.00];
    style[imgui::StyleColor::PlotLines] = [0.40, 0.39, 0.38, 0.63];
    style[imgui::StyleColor::PlotLinesHovered] = [0.25, 1.00, 0.00, 1.00];
    style[imgui::StyleColor::PlotHistogram] = [0.40, 0.39, 0.38, 0.63];
    style[imgui::StyleColor::PlotHistogramHovered] = [0.25, 1.00, 0.00, 1.00];
    style[imgui::StyleColor::TextSelectedBg] = [0.25, 1.00, 0.00, 0.43];
    style[imgui::StyleColor::ModalWindowDimBg] = [1.00, 0.98, 0.95, 0.73];
}

// Mashup made from the ones above. Mostly Extasy's + Rakiri's
pub fn set_eel_style(style: &mut Style) {
    set_extasy_style(style);

    style[imgui::StyleColor::CheckMark] = [0.28, 0.56, 1.00, 1.00];
    style[imgui::StyleColor::PlotHistogram] = [0.90, 0.70, 0.00, 1.00];
    style[imgui::StyleColor::HeaderHovered] = style[imgui::StyleColor::ButtonHovered];

    style[imgui::StyleColor::Button] = [0.20, 0.25, 0.29, 1.00];
    style[imgui::StyleColor::ButtonHovered] = [0.28, 0.56, 1.00, 1.00];
    style[imgui::StyleColor::ButtonActive] = [0.06, 0.53, 0.98, 1.00];

    // For whatever reason, this thing's transparency overwrites the whole
    // application background, rather than add/multiply
    style[imgui::StyleColor::ModalWindowDimBg] = [0.14, 0.14, 0.14, 0.80];
}

pub fn set_imnodes_style(style: &mut ImNodesStyle) {
    /*
    Originals:
        GridBackground = 0xC8322828
        MiniMapBackground = 0x96191919
        MiniMapBackgroundHovered = 0xC8191919
        MiniMapNodeBackground = 0x64C8C8C8
        MiniMapNodeBackgroundHovered = 0xFFC8C8C8
        MiniMapNodeBackgroundSelected = 0xFFC8C8C8
    */

    // Removes some of the excessive transparency

    style.Colors[imnodes::ColorStyle::GridBackground as usize] = 0xDE_32_28_28;
    style.Colors[imnodes::ColorStyle::MiniMapBackground as usize] = 0xDE_19_19_19;
    style.Colors[imnodes::ColorStyle::MiniMapBackgroundHovered as usize] = 0xDE_19_19_19;
    style.Colors[imnodes::ColorStyle::MiniMapNodeBackground as usize] = 0xDE_C8_C8_C8;
    style.Colors[imnodes::ColorStyle::MiniMapNodeBackgroundHovered as usize] = 0xDE_C8_C8_C8;
    style.Colors[imnodes::ColorStyle::MiniMapNodeBackgroundSelected as usize] = 0xDE_C8_C8_C8;
}