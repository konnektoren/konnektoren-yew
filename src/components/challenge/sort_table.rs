use konnektoren_core::challenges::{ChallengeResult, SortTable, SortTableRow};
use konnektoren_core::commands::{ChallengeCommand, Command};
use konnektoren_core::events::Event;
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
            #[cfg(feature = "csr")]
            dragged_cell.set(Some((row_index, col_index)));
        })
    };

    let handle_drag_over = Callback::from(move |e: DragEvent| {
        #[cfg(feature = "csr")]
        e.prevent_default();
    });

    let handle_touch_start = {
        let dragged_cell = dragged_cell.clone();
        Callback::from(move |e: TouchEvent| {
            #[cfg(feature = "csr")]
            {
                use wasm_bindgen::JsCast;
                use web_sys::Element;

                e.prevent_default();
                if let Some(target) = e.target() {
                    if let Ok(element) = target.dyn_into::<Element>() {
                        if let (Ok(row_index), Ok(col_index)) = (
                            element
                                .get_attribute("data-row-index")
                                .unwrap_or_default()
                                .parse::<usize>(),
                            element
                                .get_attribute("data-col-index")
                                .unwrap_or_default()
                                .parse::<usize>(),
                        ) {
                            dragged_cell.set(Some((row_index, col_index)));
                        }
                    }
                }
            }
        })
    };

    let handle_touch_move = {
        let rows = rows.clone();
        let dragged_cell = dragged_cell.clone();
        Callback::from(move |e: TouchEvent| {
            #[cfg(feature = "csr")]
            {
                use wasm_bindgen::JsCast;
                use web_sys::Element;

                e.prevent_default();
                if let Some(touch) = e.touches().get(0) {
                    if let Some(target) = touch.target() {
                        if let Ok(element) = target.dyn_into::<Element>() {
                            if let (Ok(target_row_index), Ok(target_col_index)) = (
                                element
                                    .get_attribute("data-row-index")
                                    .unwrap_or_default()
                                    .parse::<usize>(),
                                element
                                    .get_attribute("data-col-index")
                                    .unwrap_or_default()
                                    .parse::<usize>(),
                            ) {
                                if let Some((source_row_index, source_col_index)) = *dragged_cell {
                                    let mut updated_rows = (*rows).clone();
                                    if source_row_index != target_row_index
                                        || source_col_index != target_col_index
                                    {
                                        if source_col_index
                                            < updated_rows[source_row_index].values.len()
                                            && target_col_index
                                                < updated_rows[target_row_index].values.len()
                                        {
                                            let src_value = updated_rows[source_row_index].values
                                                [source_col_index]
                                                .clone();
                                            updated_rows[source_row_index].values
                                                [source_col_index] = updated_rows[target_row_index]
                                                .values[target_col_index]
                                                .clone();
                                            updated_rows[target_row_index].values
                                                [target_col_index] = src_value;
                                            rows.set(updated_rows);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    };

    let handle_cell_click = {
        let rows = rows.clone();
        let selected_cell = selected_cell.clone();
        Callback::from(move |(row_index, col_index): (usize, usize)| {
            #[cfg(feature = "csr")]
            {
                if let Some((selected_row_index, selected_col_index)) = *selected_cell {
                    if selected_row_index != row_index || selected_col_index != col_index {
                        let mut updated_rows = (*rows).clone();
                        if selected_row_index < updated_rows.len()
                            && row_index < updated_rows.len()
                            && selected_col_index < updated_rows[selected_row_index].values.len()
                            && col_index < updated_rows[row_index].values.len()
                        {
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
            }
        })
    };

    let handle_drop = {
        let rows = rows.clone();
        let dragged_cell = dragged_cell.clone();
        Callback::from(
            move |(e, target_row_index, target_col_index): (DragEvent, usize, usize)| {
                #[cfg(feature = "csr")]
                {
                    e.prevent_default();
                    if let Some((source_row_index, source_col_index)) = *dragged_cell {
                        let mut updated_rows = (*rows).clone();
                        if source_row_index != target_row_index
                            || source_col_index != target_col_index
                        {
                            if source_row_index < updated_rows.len()
                                && target_row_index < updated_rows.len()
                                && source_col_index < updated_rows[source_row_index].values.len()
                                && target_col_index < updated_rows[target_row_index].values.len()
                            {
                                let temp =
                                    updated_rows[source_row_index].values[source_col_index].clone();
                                updated_rows[source_row_index].values[source_col_index] =
                                    updated_rows[target_row_index].values[target_col_index].clone();
                                updated_rows[target_row_index].values[target_col_index] = temp;
                                rows.set(updated_rows);
                            }
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
                        let is_row_selected = matches!(*selected_cell, Some((selected_row, _)) if selected_row == row_index);

                        html! {
                            <tr class={classes!(
                                "sort-table__body-row",
                                is_row_selected.then_some("sort-table__body-row--selected")
                            )}>
                                { for row.values.iter().enumerate().map(|(col_index, value)| {
                                    let is_selected = matches!(*selected_cell,
                                        Some((selected_row, selected_col)) if selected_row == row_index && selected_col == col_index);
                                    let is_dragging = matches!(*dragged_cell,
                                        Some((drag_row, drag_col)) if drag_row == row_index && drag_col == col_index);

                                    html! {
                                        <td
                                            class={classes!(
                                                "sort-table__body-cell",
                                                is_selected.then_some("sort-table__body-cell--selected"),
                                                is_dragging.then_some("sort-table__body-cell--dragging")
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
