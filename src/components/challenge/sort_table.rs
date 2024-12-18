use konnektoren_core::challenges::{ChallengeResult, SortTable, SortTableRow};
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::events::Event;
use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Element, TouchEvent};
use yew::prelude::*;

#[derive(Properties, PartialEq, Default)]
pub struct SortTableComponentProps {
    pub challenge: SortTable,
    #[prop_or_default]
    pub on_command: Option<Callback<Command>>,
    #[prop_or_default]
    pub on_event: Option<Callback<Event>>,
}

fn shuffle(rows: &[SortTableRow]) -> Vec<SortTableRow> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();

    let mut rows = rows.to_owned();

    let mut columns: Vec<Vec<String>> = vec![vec![]; rows[0].values.len()];

    for row in &rows {
        for (col_idx, value) in row.values.iter().enumerate() {
            columns[col_idx].push(value.clone());
        }
    }

    for columns in columns.iter_mut().skip(1) {
        columns.shuffle(&mut rng);
    }

    for (row_idx, row) in rows.iter_mut().enumerate() {
        for (col_idx, _) in columns.iter().enumerate().skip(1) {
            row.values[col_idx].clone_from(&columns[col_idx][row_idx]);
        }
    }

    rows
}

#[function_component(SortTableComponent)]
pub fn sort_table_comp(props: &SortTableComponentProps) -> Html {
    let SortTableComponentProps {
        challenge,
        on_command,
        ..
    } = props;

    let rows = use_state(|| shuffle(&challenge.rows));
    let dragged_cell = use_state(|| None::<(usize, usize)>);
    let selected_cell = use_state(|| None::<(usize, usize)>);

    let handle_drag_start = {
        let dragged_cell = dragged_cell.clone();
        Callback::from(move |(row_index, col_index): (usize, usize)| {
            dragged_cell.set(Some((row_index, col_index)));
        })
    };

    let handle_drag_over = Callback::from(|event: DragEvent| {
        event.prevent_default();
    });

    let handle_touch_start = {
        let dragged_cell = dragged_cell.clone();
        Callback::from(move |event: TouchEvent| {
            event.prevent_default();
            let target = event.target().unwrap();
            let element = target.dyn_into::<Element>().unwrap();
            let row_index = element
                .get_attribute("data-row-index")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let col_index = element
                .get_attribute("data-col-index")
                .unwrap()
                .parse::<usize>()
                .unwrap();
            dragged_cell.set(Some((row_index, col_index)));
        })
    };

    let handle_touch_move = {
        let rows = rows.clone();
        let dragged_cell = dragged_cell.clone();
        Callback::from(move |event: TouchEvent| {
            event.prevent_default();
            if let Some(touch) = event.touches().get(0) {
                let target = touch.target().unwrap();
                let element = target.dyn_into::<Element>().unwrap();
                let target_row_index = element
                    .get_attribute("data-row-index")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                let target_col_index = element
                    .get_attribute("data-col-index")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                if let Some((source_row_index, source_col_index)) = *dragged_cell {
                    let mut updated_rows = (*rows).clone();

                    if source_row_index != target_row_index || source_col_index != target_col_index
                    {
                        if source_col_index < updated_rows[source_row_index].values.len()
                            && target_col_index < updated_rows[target_row_index].values.len()
                        {
                            let src_value = updated_rows[source_row_index]
                                .values
                                .remove(source_col_index);
                            let tgt_value = updated_rows[target_row_index]
                                .values
                                .remove(target_col_index);

                            updated_rows[source_row_index]
                                .values
                                .insert(source_col_index, tgt_value);
                            updated_rows[target_row_index]
                                .values
                                .insert(target_col_index, src_value);

                            rows.set(updated_rows);
                        }
                    }
                }
            }
        })
    };

    // Inside handle_cell_click
    let handle_cell_click = {
        let rows = rows.clone();
        let selected_cell = selected_cell.clone();
        Callback::from(move |(row_index, col_index): (usize, usize)| {
            if let Some((selected_row_index, selected_col_index)) = *selected_cell {
                if selected_row_index != row_index || selected_col_index != col_index {
                    let mut updated_rows = (*rows).clone();

                    // Safely check bounds before performing swap
                    if selected_row_index < updated_rows.len()
                        && row_index < updated_rows.len()
                        && selected_col_index < updated_rows[selected_row_index].values.len()
                        && col_index < updated_rows[row_index].values.len()
                    {
                        // Swap values directly instead of remove/insert
                        let temp =
                            updated_rows[selected_row_index].values[selected_col_index].clone();
                        updated_rows[selected_row_index].values[selected_col_index] =
                            updated_rows[row_index].values[col_index].clone();
                        updated_rows[row_index].values[col_index] = temp;

                        rows.set(updated_rows);
                    }
                }
                selected_cell.set(None);
            } else {
                selected_cell.set(Some((row_index, col_index)));
            }
        })
    };

    // Similarly update handle_drop and handle_touch_move
    let handle_drop = {
        let rows = rows.clone();
        let dragged_cell = dragged_cell.clone();
        Callback::from(
            move |(event, target_row_index, target_col_index): (DragEvent, usize, usize)| {
                event.prevent_default();
                if let Some((source_row_index, source_col_index)) = *dragged_cell {
                    let mut updated_rows = (*rows).clone();

                    if source_row_index != target_row_index || source_col_index != target_col_index
                    {
                        if source_row_index < updated_rows.len()
                            && target_row_index < updated_rows.len()
                            && source_col_index < updated_rows[source_row_index].values.len()
                            && target_col_index < updated_rows[target_row_index].values.len()
                        {
                            // Swap values directly
                            let temp =
                                updated_rows[source_row_index].values[source_col_index].clone();
                            updated_rows[source_row_index].values[source_col_index] =
                                updated_rows[target_row_index].values[target_col_index].clone();
                            updated_rows[target_row_index].values[target_col_index] = temp;

                            rows.set(updated_rows);
                        }
                    }
                }
            },
        )
    };

    let handle_finish = {
        let on_command = on_command.clone();
        let rows = rows.clone();
        Callback::from(move |_| {
            let result = ChallengeResult::SortTable((*rows).clone());
            if let Some(on_command) = on_command.as_ref() {
                let command = Command::Challenge(ChallengeCommand::Finish(Some(result)));
                on_command.emit(command);
            }
        })
    };

    html! {
        <div class="sort-table">
            <h1 class="sort-table__title">{ &challenge.name }</h1>
            <p class="sort-table__description">{ &challenge.description }</p>
            <table class="sort-table__table">
                <thead class="sort-table__header">
                    <tr>
                        { for challenge.columns.iter().map(|column| html! {
                            <th class="sort-table__header-cell">{ column.title.to_string() }</th>
                        }) }
                    </tr>
                </thead>
                <tbody class="sort-table__body">
                    { for rows.iter().enumerate().map(|(row_index, row)| {
                        // Check if any cell in this row is selected
                        let is_row_selected = match *selected_cell {
                            Some((selected_row, _)) => selected_row == row_index,
                            None => false,
                        };

                        html! {
                            <tr class={classes!(
                                "sort-table__body-row",
                                if is_row_selected { Some("sort-table__body-row--selected") } else { None }
                            )}>
                                { for row.values.iter().enumerate().map(|(col_index, value)| {
                                    let is_selected = match *selected_cell {
                                        Some((selected_row, selected_col)) =>
                                            selected_row == row_index && selected_col == col_index,
                                        None => false,
                                    };
                                    let is_dragging = match *dragged_cell {
                                        Some((drag_row, drag_col)) =>
                                            drag_row == row_index && drag_col == col_index,
                                        None => false,
                                    };
                                    html! {
                                        <td
                                            class={classes!(
                                                "sort-table__body-cell",
                                                if is_selected { Some("sort-table__body-cell--selected") } else { None },
                                                if is_dragging { Some("sort-table__body-cell--dragging") } else { None }
                                            )}
                                            draggable="true"
                                            data-row-index={row_index.to_string()}
                                            data-col-index={col_index.to_string()}
                                            ondragstart={handle_drag_start.reform(move |_| (row_index, col_index))}
                                            ondragover={handle_drag_over.clone()}
                                            ondrop={handle_drop.reform(move |event| (event, row_index, col_index))}
                                            ontouchstart={handle_touch_start.clone()}
                                            ontouchmove={handle_touch_move.clone()}
                                            onclick={handle_cell_click.reform(move |_| (row_index, col_index))}
                                        >
                                            { value }
                                        </td>
                                    }
                                }) }
                            </tr>
                        }
                    }) }
                </tbody>
            </table>
            <div class="sort-table__actions">
                <button class="sort-table__actions-button" onclick={handle_finish}>
                    {"Finish"}
                </button>
            </div>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SortTableComponent,
        SortTableComponentProps {
            challenge: SortTable::default(),
            ..Default::default()
        },
    );
}
