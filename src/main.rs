use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum CellStatus {
    Empty,
    _Black,
    _White,
}

#[derive(Properties, PartialEq)]
struct BoardProps {
    rows: usize,
    columns: usize,
}

#[function_component(Board)]
fn board(BoardProps { rows, columns }: &BoardProps) -> Html {
    let cells = vec![CellStatus::Empty; columns * rows];

    html!(<div class="board">
        {cells
        .iter().enumerate()
        .map(|(idx, _)| {
            html!(<div key={idx} class="cell"></div>)
        })
        .collect::<Html>()}
    </div>)
}

#[function_component(App)]
pub fn app() -> Html {
    const COLUMNS: usize = 15;
    const ROWS: usize = 15;
    html! {
        <main>
            <Board rows={ROWS} columns={COLUMNS}/>
        </main>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::trace!("Initializing yew...");
    yew::Renderer::<App>::new().render();
}
