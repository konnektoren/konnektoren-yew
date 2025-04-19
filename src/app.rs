use crate::components::{challenge::ChallengeComponent, game_path::GamePathComponent};
use crate::prelude::create_i18n_config;

#[cfg(feature = "chat")]
use crate::components::chat::ChatComponent;

#[cfg(feature = "yew-preview")]
use crate::components::navigation::menu::preview::ExampleMenu;

#[cfg(feature = "yew-preview")]
use crate::prelude::*;

#[cfg(feature = "yew-preview")]
use yew_router::BrowserRouter;

#[cfg(feature = "storage")]
use crate::components::profile::{ProfileConfigComponent, ProfilePointsComponent};

use crate::model::DefaultSessionInitializer;
use crate::prelude::{BrowserCoordinate, ChallengeIndex, MapComponent, ProfilePointsManager};
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
    #[cfg(feature = "yew-preview")]
    let groups: ComponentList = vec![
        create_component_group!(
            "Challenge",
            #[cfg(feature = "effects")]
            BlinkAnimation::preview(),
            MultipleChoiceComponent::preview(),
            MultipleChoiceCircleComponent::preview(),
            SortTableComponent::preview(),
            ContextualChoiceComponent::preview(),
            ContextualChoiceResultComponent::preview(),
            GapFillComponent::preview(),
            OrderingComponent::preview(),
            OrderingResultComponent::preview(),
            PlaceholderComponent::preview(),
            ChallengeComponent::preview(),
            ChallengeActionsComponent::preview(),
            ChallengeConfigComponent::preview(),
            ChallengeInfoComponent::preview(),
            ChallengeReviewComponent::preview(),
            ChallengeRatingComponent::preview(),
            ChallengeTimerComponent::preview(),
            ChallengesSummaryComp::preview(),
        ),
        create_component_group!(
            "Analytics",
            SuccessRateComponent::preview(),
            AverageTimeTakenComponent::preview(),
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
            QuestionComponent::preview(),
            LeaderboardComp::preview()
        ),
        create_component_group!(
            "UI Components",
            RatingStarsComponent::preview(),
            InformativeComponent::preview(),
            InformativeMarkdownComponent::preview(),
            #[cfg(feature = "effects")]
            ReadText::preview(),
            TranslateComponent::preview(),
            SwipeComponent::preview(),
            VideoComponent::preview()
        ),
        create_component_group!(
            "Settings",
            DomainSelectorComponent::preview(),
            OptionsComponent::preview(),
            SelectLanguage::preview(),
            SelectLevelComp::preview(),
            SettingsComponent::preview(),
            MusicComponent::preview(),
            MusicConfig::preview(),
            SoundConfig::preview(),
            SelectDesign::preview(),
            SelectTheme::preview(),
            AppVersionComponent::preview(),
            #[cfg(feature = "sbom")]
            AppDependenciesComponent::preview()
        ),
        create_component_group!(
            "Certificates",
            AchievementComponent::preview(),
            AchievementsComponent::preview(),
            CertificateComponent::preview(),
            CertificateImageComponent::preview()
        ),
        create_component_group!("Navigation", ExampleMenu::preview(),),
        #[cfg(feature = "marketplace")]
        create_component_group!(
            "Marketplace",
            ProductComponent::preview(),
            ProductCatalogComponent::preview(),
            ShoppingCartComponent::preview(),
            CartBadgeComponent::preview(),
            #[cfg(feature = "csr")]
            WalletComponent::<crate::components::marketplace::wallet::ton::TonWalletProvider>::preview(),
            #[cfg(feature = "solana")]
            WalletComponent::<crate::components::marketplace::wallet::solana::SolanaWalletProvider>::preview(),
        ),
        create_component_group!(
            "Misc",
            SharePageComp::preview(),
            create_component_item!("Example", Example, vec![("default", ())]),
            Badge::preview(),
            SocialLinks::preview(),
            Logo::preview(),
            #[cfg(feature = "chat")]
            ChatComponent::preview(),
            StatusMessage::preview(),
            FeedbackPopup::preview(),
            BuyMeCoffeeComponent::preview(),
            AdvertisementComponent::preview(),
        ),
        #[cfg(feature = "tour")]
        create_component_group!("Tour", crate::prelude::TourConfig::preview(), TourButton::preview()),
    ];

    let i18n_config = create_i18n_config();

    #[cfg(debug_assertions)]
    log::info!(
        "Initialized I18nConfig with default language: {}",
        i18n_config.default_language.native_name()
    );

    let storage = LocalStorage::new(None);
    let session_initilizer = DefaultSessionInitializer;
    let repository_config = create_repositories(storage, Arc::new(session_initilizer));

    #[cfg(feature = "yew-preview")]
    html! {
        <BrowserRouter>
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
        </BrowserRouter>
    }
    #[cfg(not(feature = "yew-preview"))]
    html! {
        <Example />
    }
}
