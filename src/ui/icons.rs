use druid::widget::SvgData;

pub fn modified_icon() -> SvgData {
    include_str!("../../resources/icons/diff-modified.svg")
        .parse::<SvgData>()
        .unwrap()
}
pub fn removed_icon() -> SvgData {
    include_str!("../../resources/icons/diff-removed.svg")
        .parse::<SvgData>()
        .unwrap()
}

pub fn copy_icon() -> SvgData {
    include_str!("../../resources/icons/diff-copy.svg")
        .parse::<SvgData>()
        .unwrap()
}
pub fn added_icon() -> SvgData {
    include_str!("../../resources/icons/diff-added.svg")
        .parse::<SvgData>()
        .unwrap()
}
pub fn connect_icon() -> SvgData {
    include_str!("../../resources/icons/diff-connect.svg")
        .parse::<SvgData>()
        .unwrap()
}

pub fn broker_list() -> SvgData {
    include_str!("../../resources/icons/wenjianjia.svg")
        .parse::<SvgData>()
        .unwrap()
}
pub fn broker_info() -> SvgData {
    include_str!("../../resources/icons/wendangxiugai.svg")
        .parse::<SvgData>()
        .unwrap()
}
pub fn tips() -> SvgData {
    include_str!("../../resources/icons/zhongyaotishi.svg")
        .parse::<SvgData>()
        .unwrap()
}
