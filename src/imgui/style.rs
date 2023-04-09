use imgui::sys::{ImGuiStyle, ImVec4, ImVec2};

struct Colors<'a>(&'a mut [ImVec4; imgui::StyleColor::COUNT]);

impl<'a> std::ops::Index<imgui::StyleColor> for Colors<'a> {
    type Output = ImVec4;
    #[inline]
    fn index(& self, index: imgui::StyleColor) -> & Self::Output {
        & self.0[index as usize]
    }
}

impl<'a> std::ops::IndexMut<imgui::StyleColor> for Colors<'a> {
    #[inline]
    fn index_mut(&mut self, index: imgui::StyleColor) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

pub fn set_raikiri_style(_style: &mut ImGuiStyle) {

    // My tweaks
    _style.PopupRounding = 4.0;
    _style.ChildRounding = _style.PopupRounding;
    _style.WindowRounding = _style.ChildRounding;
    _style.WindowBorderSize = 1.0;
    _style.PopupBorderSize = 0.5;
    _style.ChildBorderSize = _style.PopupBorderSize;

    // imGuiIO.Fonts->AddFontFromFileTTF("../data/Fonts/Ruda-Bold.ttf", 15.0,
    // &config);
    _style.ScrollbarRounding = 4.0;
    _style.GrabRounding = _style.ScrollbarRounding;
    _style.FrameRounding = _style.GrabRounding;

    let mut colors = Colors(&mut _style.Colors);
    colors[imgui::StyleColor::Text]                  = ImVec4::new(0.95, 0.96, 0.98, 1.00);
    colors[imgui::StyleColor::TextDisabled]          = ImVec4::new(0.36, 0.42, 0.47, 1.00);
    colors[imgui::StyleColor::WindowBg]              = ImVec4::new(0.11, 0.15, 0.17, 1.00);
    colors[imgui::StyleColor::ChildBg]               = ImVec4::new(0.15, 0.18, 0.22, 1.00);
    colors[imgui::StyleColor::PopupBg]               = ImVec4::new(0.08, 0.08, 0.08, 0.94);
    colors[imgui::StyleColor::Border]                = ImVec4::new(0.08, 0.10, 0.12, 1.00);
    colors[imgui::StyleColor::BorderShadow]          = ImVec4::new(0.1, 0.1, 0.1, 1.00);
    colors[imgui::StyleColor::FrameBg]               = ImVec4::new(0.20, 0.25, 0.29, 1.00);
    colors[imgui::StyleColor::FrameBgHovered]        = ImVec4::new(0.12, 0.20, 0.28, 1.00);
    colors[imgui::StyleColor::FrameBgActive]         = ImVec4::new(0.09, 0.12, 0.14, 1.00);
    colors[imgui::StyleColor::TitleBg]               = ImVec4::new(0.09, 0.12, 0.14, 0.65);
    colors[imgui::StyleColor::TitleBgActive]         = ImVec4::new(0.08, 0.10, 0.12, 1.00);
    colors[imgui::StyleColor::TitleBgCollapsed]      = ImVec4::new(0.00, 0.00, 0.00, 0.51);
    colors[imgui::StyleColor::MenuBarBg]             = ImVec4::new(0.15, 0.18, 0.22, 1.00);
    colors[imgui::StyleColor::ScrollbarBg]           = ImVec4::new(0.02, 0.02, 0.02, 0.39);
    colors[imgui::StyleColor::ScrollbarGrab]         = ImVec4::new(0.20, 0.25, 0.29, 1.00);
    colors[imgui::StyleColor::ScrollbarGrabHovered]  = ImVec4::new(0.18, 0.22, 0.25, 1.00);
    colors[imgui::StyleColor::ScrollbarGrabActive]   = ImVec4::new(0.09, 0.21, 0.31, 1.00);
    colors[imgui::StyleColor::CheckMark]             = ImVec4::new(0.28, 0.56, 1.00, 1.00);
    colors[imgui::StyleColor::SliderGrab]            = ImVec4::new(0.28, 0.56, 1.00, 1.00);
    colors[imgui::StyleColor::SliderGrabActive]      = ImVec4::new(0.37, 0.61, 1.00, 1.00);
    colors[imgui::StyleColor::Button]                = ImVec4::new(0.20, 0.25, 0.29, 1.00);
    colors[imgui::StyleColor::ButtonHovered]         = ImVec4::new(0.28, 0.56, 1.00, 1.00);
    colors[imgui::StyleColor::ButtonActive]          = ImVec4::new(0.06, 0.53, 0.98, 1.00);
    colors[imgui::StyleColor::Header]                = ImVec4::new(0.20, 0.25, 0.29, 0.55);
    colors[imgui::StyleColor::HeaderHovered]         = ImVec4::new(0.26, 0.59, 0.98, 0.80);
    colors[imgui::StyleColor::HeaderActive]          = ImVec4::new(0.26, 0.59, 0.98, 1.00);
    colors[imgui::StyleColor::Separator]             = ImVec4::new(0.20, 0.25, 0.29, 1.00);
    colors[imgui::StyleColor::SeparatorHovered]      = ImVec4::new(0.10, 0.40, 0.75, 0.78);
    colors[imgui::StyleColor::SeparatorActive]       = ImVec4::new(0.10, 0.40, 0.75, 1.00);
    colors[imgui::StyleColor::ResizeGrip]            = ImVec4::new(0.26, 0.59, 0.98, 0.25);
    colors[imgui::StyleColor::ResizeGripHovered]     = ImVec4::new(0.26, 0.59, 0.98, 0.67);
    colors[imgui::StyleColor::ResizeGripActive]      = ImVec4::new(0.26, 0.59, 0.98, 0.95);
    colors[imgui::StyleColor::Tab]                   = ImVec4::new(0.11, 0.15, 0.17, 1.00);
    colors[imgui::StyleColor::TabHovered]            = ImVec4::new(0.26, 0.59, 0.98, 0.80);
    colors[imgui::StyleColor::TabActive]             = ImVec4::new(0.20, 0.25, 0.29, 1.00);
    colors[imgui::StyleColor::TabUnfocused]          = ImVec4::new(0.11, 0.15, 0.17, 1.00);
    colors[imgui::StyleColor::TabUnfocusedActive]    = ImVec4::new(0.11, 0.15, 0.17, 1.00);
    colors[imgui::StyleColor::PlotLines]             = ImVec4::new(0.61, 0.61, 0.61, 1.00);
    colors[imgui::StyleColor::PlotLinesHovered]      = ImVec4::new(1.00, 0.43, 0.35, 1.00);
    colors[imgui::StyleColor::PlotHistogram]         = ImVec4::new(0.90, 0.70, 0.00, 1.00);
    colors[imgui::StyleColor::PlotHistogramHovered]  = ImVec4::new(1.00, 0.60, 0.00, 1.00);
    colors[imgui::StyleColor::TextSelectedBg]        = ImVec4::new(0.26, 0.59, 0.98, 0.35);
    colors[imgui::StyleColor::DragDropTarget]        = ImVec4::new(1.00, 1.00, 0.00, 0.90);
    colors[imgui::StyleColor::NavHighlight]          = ImVec4::new(0.26, 0.59, 0.98, 1.00);
    colors[imgui::StyleColor::NavWindowingHighlight] = ImVec4::new(1.00, 1.00, 1.00, 0.70);
    colors[imgui::StyleColor::NavWindowingDimBg]     = ImVec4::new(0.80, 0.80, 0.80, 0.20);
    colors[imgui::StyleColor::ModalWindowDimBg]      = ImVec4::new(0.80, 0.80, 0.80, 0.35);
}

// Obtained from
// https://www.unknowncheats.me/forum/1547436-post1.html?s=78e3c8907fe6b443b8422ca0c24c10
pub fn set_extasy_style(_style: &mut ImGuiStyle) {

    _style.WindowPadding     = ImVec2::new(15.0, 15.0);
    _style.WindowRounding    = 5.0;
    _style.FramePadding      = ImVec2::new(5.0, 5.0);
    _style.FrameRounding     = 4.0;
    _style.ItemSpacing       = ImVec2::new(12.0, 8.0);
    _style.ItemInnerSpacing  = ImVec2::new(8.0, 6.0);
    _style.IndentSpacing     = 25.0;
    _style.ScrollbarSize     = 15.0;
    _style.ScrollbarRounding = 9.0;
    _style.GrabMinSize       = 5.0;
    _style.GrabRounding      = 3.0;

    let mut colors = Colors(&mut _style.Colors);

    colors[imgui::StyleColor::Text]                 = ImVec4::new(0.80, 0.80, 0.83, 1.00);
    colors[imgui::StyleColor::TextDisabled]         = ImVec4::new(0.24, 0.23, 0.29, 1.00);
    colors[imgui::StyleColor::WindowBg]             = ImVec4::new(0.06, 0.05, 0.07, 1.00);
    colors[imgui::StyleColor::ChildBg]              = ImVec4::new(0.07, 0.07, 0.09, 1.00);
    colors[imgui::StyleColor::PopupBg]              = ImVec4::new(0.07, 0.07, 0.09, 1.00);
    colors[imgui::StyleColor::Border]               = ImVec4::new(0.80, 0.80, 0.83, 0.88);
    colors[imgui::StyleColor::BorderShadow]         = ImVec4::new(0.92, 0.91, 0.88, 0.00);
    colors[imgui::StyleColor::FrameBg]              = ImVec4::new(0.10, 0.09, 0.12, 1.00);
    colors[imgui::StyleColor::FrameBgHovered]       = ImVec4::new(0.24, 0.23, 0.29, 1.00);
    colors[imgui::StyleColor::FrameBgActive]        = ImVec4::new(0.56, 0.56, 0.58, 1.00);
    colors[imgui::StyleColor::TitleBg]              = ImVec4::new(0.10, 0.09, 0.12, 1.00);
    colors[imgui::StyleColor::TitleBgCollapsed]     = ImVec4::new(1.00, 0.98, 0.95, 0.75);
    colors[imgui::StyleColor::TitleBgActive]        = ImVec4::new(0.07, 0.07, 0.09, 1.00);
    colors[imgui::StyleColor::MenuBarBg]            = ImVec4::new(0.10, 0.09, 0.12, 1.00);
    colors[imgui::StyleColor::ScrollbarBg]          = ImVec4::new(0.10, 0.09, 0.12, 1.00);
    colors[imgui::StyleColor::ScrollbarGrab]        = ImVec4::new(0.80, 0.80, 0.83, 0.31);
    colors[imgui::StyleColor::ScrollbarGrabHovered] = ImVec4::new(0.56, 0.56, 0.58, 1.00);
    colors[imgui::StyleColor::ScrollbarGrabActive]  = ImVec4::new(0.06, 0.05, 0.07, 1.00);
    colors[imgui::StyleColor::CheckMark]            = ImVec4::new(0.80, 0.80, 0.83, 0.31);
    colors[imgui::StyleColor::SliderGrab]           = ImVec4::new(0.80, 0.80, 0.83, 0.31);
    colors[imgui::StyleColor::SliderGrabActive]     = ImVec4::new(0.06, 0.05, 0.07, 1.00);
    colors[imgui::StyleColor::Button]               = ImVec4::new(0.10, 0.09, 0.12, 1.00);
    colors[imgui::StyleColor::ButtonHovered]        = ImVec4::new(0.24, 0.23, 0.29, 1.00);
    colors[imgui::StyleColor::ButtonActive]         = ImVec4::new(0.56, 0.56, 0.58, 1.00);
    colors[imgui::StyleColor::Header]               = ImVec4::new(0.10, 0.09, 0.12, 1.00);
    colors[imgui::StyleColor::HeaderHovered]        = ImVec4::new(0.56, 0.56, 0.58, 1.00);
    colors[imgui::StyleColor::HeaderActive]         = ImVec4::new(0.06, 0.05, 0.07, 1.00);
    colors[imgui::StyleColor::Separator]            = ImVec4::new(0.56, 0.56, 0.58, 1.00);
    colors[imgui::StyleColor::SeparatorHovered]     = ImVec4::new(0.24, 0.23, 0.29, 1.00);
    colors[imgui::StyleColor::SeparatorActive]      = ImVec4::new(0.56, 0.56, 0.58, 1.00);
    colors[imgui::StyleColor::ResizeGrip]           = ImVec4::new(0.00, 0.00, 0.00, 0.00);
    colors[imgui::StyleColor::ResizeGripHovered]    = ImVec4::new(0.56, 0.56, 0.58, 1.00);
    colors[imgui::StyleColor::ResizeGripActive]     = ImVec4::new(0.06, 0.05, 0.07, 1.00);
    colors[imgui::StyleColor::PlotLines]            = ImVec4::new(0.40, 0.39, 0.38, 0.63);
    colors[imgui::StyleColor::PlotLinesHovered]     = ImVec4::new(0.25, 1.00, 0.00, 1.00);
    colors[imgui::StyleColor::PlotHistogram]        = ImVec4::new(0.40, 0.39, 0.38, 0.63);
    colors[imgui::StyleColor::PlotHistogramHovered] = ImVec4::new(0.25, 1.00, 0.00, 1.00);
    colors[imgui::StyleColor::TextSelectedBg]       = ImVec4::new(0.25, 1.00, 0.00, 0.43);
    colors[imgui::StyleColor::ModalWindowDimBg]     = ImVec4::new(1.00, 0.98, 0.95, 0.73);
}

// Mashup made from the ones above. Mostly Extasy's + Rakiri's
pub fn set_eel_style(_style: &mut ImGuiStyle) {

    set_extasy_style(_style);

    let mut colors = Colors(&mut _style.Colors);
    colors[imgui::StyleColor::CheckMark]     = ImVec4::new(0.28, 0.56, 1.00, 1.00);
    colors[imgui::StyleColor::PlotHistogram] = ImVec4::new(0.90, 0.70, 0.00, 1.00);
    colors[imgui::StyleColor::HeaderHovered] = colors[imgui::StyleColor::ButtonHovered];
}
