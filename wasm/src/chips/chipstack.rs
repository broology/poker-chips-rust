use yew::{function_component, html, Properties};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ChipStackProps {
    pub count: u16,
    pub denomination: u16,
    pub col: u16,
    pub row: u16,
}

fn calculate_x_position(col: u16, row: u16) -> u16 {
    col * 62 + row * 31
}

fn calculate_y_position(row: u16) -> u16 {
    row * 15
}

fn denomination_to_asset_url(denomination: u16, count: u16) -> String {
    format!(
        "https://d17df73wtlc9mc.cloudfront.net/chips/{}/chip-{}-{}.svg",
        denomination, denomination, count
    )
}

#[function_component(ChipStack)]
pub fn chip_stack(props: &ChipStackProps) -> Html {
    html! {
        <div class="chip-stack-container" style={format!("left: {}px;top: {}px;",calculate_x_position(props.col,props.row), calculate_y_position(props.row))} >
                <div class="chip-stack">
                    <img class="rendered-chips" src={denomination_to_asset_url(props.denomination, props.count)} />
                </div>
        </div>
    }
}
