use crate::components::{
    challenge::ChallengeComponent, game_path::GamePathComponent, Badge, ChallengeConfigComponent,
    ChallengeInfoComponent, ChallengeTimerComponent, ContextualChoiceComponent, Logo,
    MusicComponent, MusicConfig, ProgressBar, RatingStarsComponent, SelectDesign, SelectTheme,
    SettingsComponent, SharePageComp, SocialLinks, SoundConfig, TranslateComponent,
};

#[cfg(feature = "yew-preview")]
use crate::components::navigation::menu::preview::ExampleMenu;

#[cfg(feature = "marketplace")]
use crate::components::marketplace::{
    CartBadgeComponent, ProductCatalogComponent, ProductComponent, ShoppingCartComponent,
    TonWalletComponent,
};

#[cfg(feature = "storage")]
use crate::components::profile::{ProfileConfigComponent, ProfilePointsComponent};

use crate::components::challenge::multiple_choice::MultipleChoiceComponentProps;
use crate::components::challenge::{
    MultipleChoiceCircleComponent, MultipleChoiceComponent, SortTableComponent,
};
#[cfg(feature = "certificates")]
use crate::components::{
    AchievementComponent, AchievementsComponent, CertificateComponent, CertificateImageComponent,
};
#[cfg(feature = "effects")]
use crate::effects::BlinkAnimation;
use crate::i18n::{I18nConfig, I18nProvider};
use crate::model::DefaultSessionInitializer;
use crate::prelude::{
    BrowserCoordinate, ChallengeActionsComponent, ChallengeIndex, ChallengeRatingComponent,
    ChallengeReviewComponent, DesignProvider, GameControllerProvider, InformativeComponent,
    InformativeMarkdownComponent, MapComponent, OptionsComponent, ProfilePointsManager,
    QuestionComponent, ReadText, RepositoryProvider, SelectLanguage, SelectLevelComp,
    ThemeProvider,
};
use crate::providers::create_repositories;
use crate::repository::LocalStorage;
use konnektoren_core::prelude::*;
use log;
use std::sync::Arc;
use yew::prelude::*;
#[cfg(feature = "yew-preview")]
use yew_preview::{create_component_group, create_component_item, prelude::*};

#[function_component]
pub fn Example() -> Html {
    let game = Game::default();
    let challenge: UseStateHandle<Option<Challenge>> = use_state(|| None);

    let new_challenge_cb = {
        let game = game.clone();
        let challenge = challenge.clone();
        Callback::from(move |challenge_config: ChallengeConfig| {
            match game.create_challenge(&challenge_config.id) {
                Ok(c) => challenge.set(Some(c)),
                Err(err) => log::error!("Error creating challenge: {:?}", err),
            }
        })
    };

    let on_map_challenge_cb = {
        let game = game.clone();
        let challenge = challenge.clone();
        Callback::from(
            move |(challenge_index, coords): (Option<ChallengeIndex>, BrowserCoordinate)| {
                let x = coords.0;
                let y = coords.1;
                if let Some(challenge_index) = challenge_index {
                    log::info!("Challenge index: {}, x: {}, y: {}", challenge_index, x, y);
                    if let Some(challenge_config) =
                        game.game_paths[0].challenges.get(challenge_index)
                    {
                        match game.create_challenge(&challenge_config.id) {
                            Ok(c) => challenge.set(Some(c)),
                            Err(_) => log::error!("Challenge creation failed"),
                        }
                    }
                } else {
                    log::error!("Challenge not found");
                }
            },
        )
    };

    let profile_config_component = {
        #[cfg(feature = "storage")]
        html! {<ProfileConfigComponent />}
        #[cfg(not(feature = "storage"))]
        html! {<></>}
    };

    let profile_points_component = {
        #[cfg(feature = "storage")]
        html! {<ProfilePointsManager><ProfilePointsComponent profile={PlayerProfile::default()} /></ProfilePointsManager>}
        #[cfg(not(feature = "storage"))]
        html! {<></>}
    };
    html! {
        <div>
            {profile_config_component}
            {profile_points_component}
            <GamePathComponent game_path={game.game_paths[0].clone()} on_challenge_config={new_challenge_cb} />
            {
                if let Some(ref challenge) = *challenge {
                    html! { <ChallengeComponent challenge={challenge.clone()} /> }
                } else {
                    html! {}
                }
            }
            <MapComponent
                game_path={game.game_paths[0].clone()}
                current_challenge={0}
                on_select_challenge={on_map_challenge_cb}
            />
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    let game = Game::default();
    let default_challenge = game.create_challenge("konnektoren-1").unwrap();
    let default_multiple_choice: MultipleChoice = match &default_challenge.challenge_type {
        ChallengeType::MultipleChoice(multiple_choice) => multiple_choice.clone(),
        _ => unreachable!(),
    };

    let i18n_config = I18nConfig::default();

    #[cfg(feature = "yew-preview")]
    let groups: ComponentList = vec![
        create_component_group!(
            "Challenge",
            BlinkAnimation::preview(),
            create_component_item!(
                "MultipleChoiceComponent",
                MultipleChoiceComponent,
                vec![(
                    "default",
                    MultipleChoiceComponentProps {
                        challenge: default_multiple_choice.clone(),
                        ..Default::default()
                    }
                )]
            ),
            create_component_item!(
                "MultipleChoiceCircleComponent",
                MultipleChoiceCircleComponent,
                vec![(
                    "default",
                    MultipleChoiceComponentProps {
                        challenge: default_multiple_choice,
                        ..Default::default()
                    }
                )]
            ),
            SortTableComponent::preview(),
            ContextualChoiceComponent::preview(),
            ChallengeComponent::preview(),
            ChallengeActionsComponent::preview(),
            ChallengeConfigComponent::preview(),
            ChallengeInfoComponent::preview(),
            ChallengeReviewComponent::preview(),
            ChallengeRatingComponent::preview(),
            ChallengeTimerComponent::preview()
        ),
        create_component_group!(
            "Profile",
            ProfileConfigComponent::preview(),
            ProfilePointsComponent::preview()
        ),
        create_component_group!(
            "Game",
            MapComponent::preview(),
            GamePathComponent::preview(),
            ProgressBar::preview(),
            QuestionComponent::preview()
        ),
        create_component_group!(
            "UI Components",
            RatingStarsComponent::preview(),
            InformativeComponent::preview(),
            InformativeMarkdownComponent::preview(),
            ReadText::preview(),
            TranslateComponent::preview()
        ),
        create_component_group!(
            "Settings",
            OptionsComponent::preview(),
            SelectLanguage::preview(),
            SelectLevelComp::preview(),
            SettingsComponent::preview(),
            MusicComponent::preview(),
            MusicConfig::preview(),
            SoundConfig::preview(),
            SelectDesign::preview(),
            SelectTheme::preview()
        ),
        create_component_group!(
            "Certificates",
            AchievementComponent::preview(),
            AchievementsComponent::preview(),
            CertificateComponent::preview(),
            CertificateImageComponent::preview()
        ),
        create_component_group!("Navigation", ExampleMenu::preview(),),
        create_component_group!(
            "Marketplace",
            ProductComponent::preview(),
            ProductCatalogComponent::preview(),
            ShoppingCartComponent::preview(),
            CartBadgeComponent::preview(),
            TonWalletComponent::preview()
        ),
        create_component_group!(
            "Misc",
            SharePageComp::preview(),
            create_component_item!("Example", Example, vec![("default", ())]),
            Badge::preview(),
            SocialLinks::preview(),
            Logo::preview()
        ),
    ];

    let storage = LocalStorage::new(None);
    let session_initilizer = DefaultSessionInitializer;
    let repository_config = create_repositories(storage, Arc::new(session_initilizer));

    #[cfg(feature = "yew-preview")]
    html! {
        <RepositoryProvider config={repository_config}>
        <ThemeProvider>
        <DesignProvider>
        <I18nProvider config={i18n_config}>
            <GameControllerProvider>
            <div style="
                font-family: Arial, sans-serif;
                height: 100vh;
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ">
                <div style="
                    padding: 10px;
                    background-color: #f8f8f8;
                    border-bottom: 1px solid #ccc;
                    flex-shrink: 0;
                ">
                    <h1 style="text-align: center;">
                        { "Konnektoren Yew Components" }
                    </h1>
                </div>
                <div style="flex: 1; overflow: hidden;">
                    <PreviewPage {groups} />
                </div>
                <div>
                    <SelectTheme />
                    <SelectDesign />
                </div>
            </div>
            </GameControllerProvider>
        </I18nProvider>
        </DesignProvider>
        </ThemeProvider>
        </RepositoryProvider>
    }
    #[cfg(not(feature = "yew-preview"))]
    html! {
        <Example />
    }
}
