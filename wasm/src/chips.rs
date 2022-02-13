use round::round_down;
use yew::{html, Component, Context, Html, Properties};

const DENOMINATIONS: [u16; 11] = [5000, 2500, 1000, 500, 250, 100, 50, 25, 10, 5, 1];
const MAX_CHIPS_PER_STACK: u16 = 10;
const STACKS_PER_ROW: u16 = 4;

mod chipstack;
use chipstack::{ChipStack, ChipStackProps};

#[derive(Properties, Clone, PartialEq)]
pub struct ChipsProps {
    pub amount: u64,
}

pub struct Chips {
    data: Vec<ChipStackProps>,
}

impl Chips {
    pub fn amount_to_chip_stack_props(amount: u64) -> Vec<ChipStackProps> {
        let mut chip_stacks: Vec<ChipStackProps> = Vec::from([]);

        let mut total: f64 = amount as f64;
        for d in DENOMINATIONS.iter() {
            let count = round_down(total as f64 / *d as f64, 0);
            let delta = *d as f64 * count as f64;
            total -= delta;

            // * Build full stacks
            let stacks = round_down(count / MAX_CHIPS_PER_STACK as f64, 0) as u16;

            for _ in 0..stacks {
                chip_stacks.push(ChipStackProps {
                    count: MAX_CHIPS_PER_STACK,
                    denomination: *d,
                    col: chip_stacks.len() as u16 % STACKS_PER_ROW,
                    row: round_down(chip_stacks.len() as f64 / STACKS_PER_ROW as f64, 0) as u16,
                });
            }

            // * Build the remainder stack
            let remainder = count as u16 % MAX_CHIPS_PER_STACK;
            if remainder > 0 {
                chip_stacks.push(ChipStackProps {
                    count: remainder,
                    denomination: *d,
                    col: chip_stacks.len() as u16 % STACKS_PER_ROW,
                    row: round_down(chip_stacks.len() as f64 / STACKS_PER_ROW as f64, 0) as u16,
                });
            }
        }

        chip_stacks
    }
}

impl Component for Chips {
    type Message = ();
    type Properties = ChipsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: Chips::amount_to_chip_stack_props(ctx.props().amount),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.data = Chips::amount_to_chip_stack_props(ctx.props().amount);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let props_to_chip_stack = |props: &ChipStackProps| {
            html! {
                <ChipStack row={props.row} col={props.col} denomination={props.denomination} count={props.count}/>
            }
        };

        html! {
            <div class="chips" >
                { for self.data.iter().map(props_to_chip_stack) }
            </div>
        }
    }
}
