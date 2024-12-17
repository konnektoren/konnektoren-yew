use crate::providers::use_profile_repository;
use crate::repository::PROFILE_STORAGE_KEY;
use konnektoren_core::prelude::PlayerProfile;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(ProfileConfigComponent)]
pub fn profile_config_component() -> Html {
    let profile_repository = use_profile_repository();
    let profile = use_state(|| PlayerProfile::default());
    let name = use_state(|| profile.name.clone());

    {
        let profile = profile.clone();
        let name = name.clone();
        let profile_repository = profile_repository.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(Some(loaded_profile)) =
                    profile_repository.get_profile(PROFILE_STORAGE_KEY).await
                {
                    profile.set(loaded_profile.clone());
                    name.set(loaded_profile.name);
                }
            });
            || ()
        });
    }

    let on_name_change = {
        let name = name.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            name.set(input.value());
        })
    };

    let on_save = {
        let name = name.clone();
        let profile = profile.clone();
        let profile_repository = profile_repository.clone();
        Callback::from(move |_| {
            let mut updated_profile = (*profile).clone();
            updated_profile.name = (*name).clone();

            wasm_bindgen_futures::spawn_local({
                let profile_repository = profile_repository.clone();
                let profile = profile.clone();
                let updated_profile = updated_profile.clone();
                async move {
                    if let Ok(_) = profile_repository
                        .update_profile(PROFILE_STORAGE_KEY, &updated_profile)
                        .await
                    {
                        profile.set(updated_profile);
                    }
                }
            });
        })
    };

    let has_changes = {
        let name = name.clone();
        let initial_name = profile.name.clone();
        move || *name != initial_name
    };

    let save_button = if has_changes() {
        html! {
            <button onclick={on_save}>{ "Save" }</button>
        }
    } else {
        html! {}
    };

    html! {
        <div class="profile-config">
            <h2>{ "Player Profile" }</h2>

            <label for="name">{ "Name" }</label>
            <input id="name" type="text" value={(*name).clone()} oninput={on_name_change} />
            { save_button }
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(ProfileConfigComponent, (),);
}
