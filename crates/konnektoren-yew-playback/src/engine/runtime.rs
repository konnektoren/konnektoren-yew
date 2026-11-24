//! Browser runtime: drives a [`PlaybackEngine`] with timers and DOM actions.
use super::{PlaybackEngine, PlaybackState, PlaybackTarget, PlaybackToken, use_playback_target};
use crate::model::{Excursion, Step};
use std::rc::Rc;
use yew::prelude::*;

/// Work started by a [`StepRunner`]. Dropping it cancels the work.
#[must_use = "dropping the task cancels it"]
pub struct StepTask(Option<Box<dyn FnOnce()>>);

impl StepTask {
    /// Run `cancel` when the step ends before `done` was called, e.g. on skip,
    /// stop, speed change or unmount. Also runs after completion, so it must be idempotent.
    pub fn new(cancel: impl FnOnce() + 'static) -> Self {
        Self(Some(Box::new(cancel)))
    }

    /// Hold any value, typically timers, until the step ends.
    pub fn hold<T: 'static>(value: T) -> Self {
        Self::new(move || drop(value))
    }
}

impl Drop for StepTask {
    fn drop(&mut self) {
        if let Some(cancel) = self.0.take() {
            cancel();
        }
    }
}

/// Application behaviour for steps, usually driven by the step's extension `X`.
///
/// Called once a step is due. Return `Some(task)` to handle the step yourself
/// and emit `done` when finished. Return `None` to run the built-in target
/// action (click, tap, fill, press) instead.
pub trait StepRunner<X>: 'static {
    fn run(&self, step: &Step<X>, done: Callback<()>) -> Option<StepTask>;
}

/// No custom behaviour.
impl<X> StepRunner<X> for () {
    fn run(&self, _: &Step<X>, _: Callback<()>) -> Option<StepTask> {
        None
    }
}

/// Timing of the browser runtime.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaybackOptions {
    /// How long a step without `delay_ms` is shown at speed 1.0.
    pub step_delay_ms: u32,
    /// Time between the click effect and the target action.
    pub action_delay_ms: u32,
    /// How long a pressed key is held.
    pub key_hold_ms: u32,
    /// Interval for checking again for a missing required target.
    pub retry_ms: u32,
}

impl Default for PlaybackOptions {
    fn default() -> Self {
        Self {
            step_delay_ms: 5_000,
            action_delay_ms: 500,
            key_hold_ms: 300,
            retry_ms: 500,
        }
    }
}

/// A click effect at a target center in viewport pixels. Use `id` as the
/// component key to replay the animation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ClickEffect {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

/// User commands for a running playback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackCommand {
    Pause,
    Resume,
    Stop,
    Faster,
    Slower,
    /// Run the current step now instead of waiting for its delay.
    Next,
}

/// Cheap, comparable handle for sending [`PlaybackCommand`]s.
#[derive(Clone, PartialEq)]
pub struct PlaybackControls(Callback<PlaybackCommand>);

impl PlaybackControls {
    pub fn send(&self, command: PlaybackCommand) {
        self.0.emit(command);
    }
    pub fn pause(&self) {
        self.send(PlaybackCommand::Pause);
    }
    pub fn resume(&self) {
        self.send(PlaybackCommand::Resume);
    }
    pub fn stop(&self) {
        self.send(PlaybackCommand::Stop);
    }
    pub fn faster(&self) {
        self.send(PlaybackCommand::Faster);
    }
    pub fn slower(&self) {
        self.send(PlaybackCommand::Slower);
    }
    pub fn next(&self) {
        self.send(PlaybackCommand::Next);
    }
}

/// Speed levels in halves: level 2 is 1.0×, the range is 0.5× to 4×.
const SPEED_LEVELS: std::ops::RangeInclusive<u8> = 1..=8;
const NORMAL_SPEED: u8 = 2;

struct Runtime<X> {
    engine: Option<PlaybackEngine<X>>,
    speed: u8,
    hurry: Option<PlaybackToken>,
    click: Option<ClickEffect>,
}

impl<X> Clone for Runtime<X> {
    fn clone(&self) -> Self {
        Self {
            engine: self.engine.clone(),
            speed: self.speed,
            hurry: self.hurry,
            click: self.click,
        }
    }
}

impl<X> Runtime<X> {
    fn new(excursion: Option<Rc<Excursion<X>>>) -> Self {
        let mut runtime = Self {
            engine: excursion.map(PlaybackEngine::new),
            speed: NORMAL_SPEED,
            hurry: None,
            click: None,
        };
        runtime.start();
        runtime
    }

    fn start(&mut self) {
        if let Some(engine) = &mut self.engine {
            engine.start();
        }
    }
}

#[cfg_attr(not(feature = "csr"), allow(dead_code))]
enum RuntimeAction<X> {
    Load(Option<Rc<Excursion<X>>>),
    Complete(PlaybackToken),
    Click(PlaybackToken, f64, f64),
    Command(PlaybackCommand),
}

impl<X: PartialEq + 'static> Reducible for Runtime<X> {
    type Action = RuntimeAction<X>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();
        match action {
            RuntimeAction::Load(excursion) => {
                let current = self.engine.as_ref().map(|engine| engine.excursion());
                let same = match (current, &excursion) {
                    (Some(a), Some(b)) => Rc::ptr_eq(a, b) || a == b,
                    (None, None) => true,
                    _ => false,
                };
                if same {
                    return self;
                }
                next = Runtime {
                    speed: self.speed,
                    ..Runtime::new(excursion)
                };
            }
            RuntimeAction::Complete(token) => {
                let Some(engine) = &mut next.engine else {
                    return self;
                };
                if !engine.complete_pending(token) {
                    return self;
                }
                next.hurry = None;
            }
            RuntimeAction::Click(token, x, y) => {
                if next.engine.as_ref().and_then(PlaybackEngine::pending) != Some(token) {
                    return self;
                }
                let id = next.click.map_or(0, |click| click.id.wrapping_add(1));
                next.click = Some(ClickEffect { id, x, y });
            }
            RuntimeAction::Command(command) => {
                let Some(engine) = &mut next.engine else {
                    return self;
                };
                match command {
                    PlaybackCommand::Pause => engine.pause(),
                    PlaybackCommand::Resume => engine.resume(),
                    PlaybackCommand::Stop => engine.stop(),
                    PlaybackCommand::Faster => {
                        next.speed = next.speed.saturating_add(1).min(*SPEED_LEVELS.end())
                    }
                    PlaybackCommand::Slower => {
                        next.speed = next.speed.saturating_sub(1).max(*SPEED_LEVELS.start())
                    }
                    PlaybackCommand::Next => next.hurry = engine.pending(),
                }
            }
        }
        next.start();
        Rc::new(next)
    }
}

/// The state of a running excursion, for rendering.
pub struct PlaybackHandle<X: 'static> {
    excursion: Option<Rc<Excursion<X>>>,
    /// Current step; stays on the last step once finished.
    pub index: usize,
    /// Completed loops of a repeating excursion.
    pub cycle: usize,
    pub total: usize,
    pub state: PlaybackState,
    /// Speed multiplier, `1.0` is normal.
    pub speed: f64,
    /// The current step's target, while it exists.
    pub target: Option<PlaybackTarget>,
    /// Cursor position in viewport percent. Stays at the last target between targets.
    pub cursor: Option<(f64, f64)>,
    pub click: Option<ClickEffect>,
    pub controls: PlaybackControls,
}

impl<X> PlaybackHandle<X> {
    pub fn excursion(&self) -> Option<&Rc<Excursion<X>>> {
        self.excursion.as_ref()
    }
    pub fn step(&self) -> Option<&Step<X>> {
        self.excursion.as_ref()?.steps.get(self.index)
    }
    /// The page in effect for the current step.
    pub fn page(&self) -> Option<&str> {
        self.excursion.as_ref()?.page_at(self.index)
    }
    /// Frame to draw when the current step asks for a highlight.
    pub fn highlight(&self) -> Option<crate::TargetRect> {
        self.step()
            .filter(|step| step.highlight)
            .and(self.target)
            .map(|target| target.rect)
    }
    /// Changes with every step and loop; use as a component key.
    pub fn step_key(&self) -> usize {
        self.cycle.wrapping_mul(self.total).wrapping_add(self.index)
    }
}

/// Play an excursion in the browser.
///
/// Each step is shown for its `delay_ms`, or for [`PlaybackOptions::step_delay_ms`]
/// scaled by the speed. Then `runner` may handle it; otherwise the built-in
/// target action runs. Required targets are awaited, optional ones skipped.
/// A new `excursion` value restarts playback. Rendering is left to the caller,
/// see the `Playback*` components.
#[hook]
pub fn use_playback<X, R>(
    excursion: Option<Rc<Excursion<X>>>,
    options: PlaybackOptions,
    runner: R,
) -> PlaybackHandle<X>
where
    X: PartialEq + 'static,
    R: StepRunner<X>,
{
    let runtime = use_reducer({
        let excursion = excursion.clone();
        move || Runtime::new(excursion)
    });
    let runner_ref = use_mut_ref(|| None::<Rc<dyn StepRunner<X>>>);
    *runner_ref.borrow_mut() = Some(Rc::new(runner));
    {
        let dispatcher = runtime.dispatcher();
        use_effect_with(excursion, move |excursion| {
            dispatcher.dispatch(RuntimeAction::Load(excursion.clone()));
        });
    }
    let controls = {
        let dispatcher = runtime.dispatcher();
        (*use_memo(dispatcher, |dispatcher| {
            let dispatcher = dispatcher.clone();
            PlaybackControls(Callback::from(move |command| {
                dispatcher.dispatch(RuntimeAction::Command(command))
            }))
        }))
        .clone()
    };

    let engine = runtime.engine.as_ref();
    let excursion = engine.map(|engine| engine.excursion().clone());
    let total = engine.map_or(0, PlaybackEngine::total);
    let index = engine.map_or(0, |engine| engine.index().min(total.saturating_sub(1)));
    let cycle = engine.map_or(0, PlaybackEngine::cycle);
    let state = engine.map_or(PlaybackState::Finished, PlaybackEngine::state);
    let speed = f64::from(runtime.speed) / f64::from(NORMAL_SPEED);
    let step = excursion.as_ref().and_then(|tour| tour.steps.get(index));
    let target = use_playback_target(
        step.and_then(|step| step.selector.clone())
            .map(AttrValue::from),
        step.is_some_and(|step| step.follow),
        cycle.wrapping_mul(total).wrapping_add(index),
    );
    let last_cursor = use_mut_ref(|| None);
    if let Some(target) = target {
        *last_cursor.borrow_mut() = Some(target.percent);
    }
    let cursor = *last_cursor.borrow();

    let active = engine
        .filter(|engine| engine.state() == PlaybackState::Waiting)
        .and_then(|engine| {
            Some((
                engine.pending()?,
                engine.excursion().clone(),
                engine.index(),
            ))
        });
    let token = active.as_ref().map(|(token, _, _)| *token);
    let hurry = token.is_some() && runtime.hurry == token;
    {
        let dispatcher = runtime.dispatcher();
        let speed = runtime.speed;
        use_effect_with((token, speed, hurry, options), move |_| {
            #[cfg(feature = "csr")]
            let guard = active.map(|(token, excursion, index)| {
                browser::run_step(browser::StepRun {
                    step: excursion,
                    index,
                    token,
                    options,
                    speed: f64::from(speed) / f64::from(NORMAL_SPEED),
                    hurry,
                    runner: runner_ref.borrow().clone(),
                    dispatcher,
                })
            });
            #[cfg(not(feature = "csr"))]
            let guard = {
                let _ = (active, options, runner_ref, dispatcher);
                None::<Box<()>>
            };
            move || drop(guard)
        });
    }

    PlaybackHandle {
        excursion,
        index,
        cycle,
        total,
        state,
        speed,
        target,
        cursor,
        click: runtime.click,
        controls,
    }
}

/// Delay before a step's action.
#[cfg(any(feature = "csr", test))]
fn step_delay<X>(step: &Step<X>, options: &PlaybackOptions, speed: f64, hurry: bool) -> u32 {
    if hurry {
        return 0;
    }
    step.delay_ms.unwrap_or_else(|| {
        crate::PlaybackTiming {
            delay_ms: options.step_delay_ms,
            speed,
        }
        .duration_ms()
        .unwrap_or(options.step_delay_ms)
    })
}

#[cfg(feature = "csr")]
mod browser {
    use super::*;
    use crate::TargetAction;
    use gloo_timers::callback::{Interval, Timeout};
    use std::cell::{Cell, RefCell};

    pub(super) struct StepRun<X: PartialEq + 'static> {
        pub step: Rc<Excursion<X>>,
        pub index: usize,
        pub token: PlaybackToken,
        pub options: PlaybackOptions,
        pub speed: f64,
        pub hurry: bool,
        pub runner: Option<Rc<dyn StepRunner<X>>>,
        pub dispatcher: UseReducerDispatcher<Runtime<X>>,
    }

    /// Everything a running step owns. Dropping it cancels the step.
    #[derive(Default)]
    pub(super) struct StepGuard {
        timeouts: RefCell<Vec<Timeout>>,
        retry: RefCell<Option<Interval>>,
        task: RefCell<Option<StepTask>>,
        held_key: RefCell<Option<(web_sys::HtmlElement, String)>>,
    }

    impl Drop for StepGuard {
        fn drop(&mut self) {
            self.timeouts.get_mut().clear();
            self.retry.get_mut().take();
            self.task.get_mut().take();
            if let Some((target, key)) = self.held_key.get_mut().take() {
                let _ = crate::send_browser_key(&target, &key, false);
            }
        }
    }

    impl StepGuard {
        fn after(&self, ms: u32, callback: impl FnOnce() + 'static) {
            self.timeouts.borrow_mut().push(Timeout::new(ms, callback));
        }
    }

    pub(super) fn run_step<X: PartialEq + 'static>(run: StepRun<X>) -> Rc<StepGuard> {
        let guard = Rc::new(StepGuard::default());
        let StepRun {
            step: excursion,
            index,
            token,
            options,
            speed,
            hurry,
            runner,
            dispatcher,
        } = run;
        let delay = step_delay(&excursion.steps[index], &options, speed, hurry);
        let weak = Rc::downgrade(&guard);
        guard.after(delay, move || {
            let Some(guard) = weak.upgrade() else { return };
            let step = &excursion.steps[index];
            let done = {
                let dispatcher = dispatcher.clone();
                Callback::from(move |()| dispatcher.dispatch(RuntimeAction::Complete(token)))
            };
            if let Some(task) = runner.and_then(|runner| runner.run(step, done.clone())) {
                *guard.task.borrow_mut() = Some(task);
                return;
            }
            let (Some(action), Some(selector)) = (step.action.clone(), step.selector.clone())
            else {
                done.emit(());
                return;
            };
            let optional = step.optional;
            let started = Rc::new(Cell::new(false));
            let attempt = {
                let weak = Rc::downgrade(&guard);
                let started = started.clone();
                move || {
                    let Some(guard) = weak.upgrade() else { return };
                    if started.get() {
                        return;
                    }
                    let target = match crate::resolve_browser_target(&selector, optional) {
                        Ok(Some(target)) => target,
                        Ok(None) => {
                            started.set(true);
                            done.emit(());
                            return;
                        }
                        // Wait for a required target to appear.
                        Err(_) => return,
                    };
                    started.set(true);
                    let bounds = target.get_bounding_client_rect();
                    dispatcher.dispatch(RuntimeAction::Click(
                        token,
                        bounds.x() + bounds.width() / 2.0,
                        bounds.y() + bounds.height() / 2.0,
                    ));
                    let weak = Rc::downgrade(&guard);
                    let done = done.clone();
                    let action = action.clone();
                    guard.after(options.action_delay_ms, move || {
                        let Some(guard) = weak.upgrade() else { return };
                        match action {
                            TargetAction::Click => target.click(),
                            TargetAction::Tap => {}
                            TargetAction::Fill(value) => {
                                let _ = crate::fill_browser_element(&target, &value);
                            }
                            TargetAction::Press(key) => {
                                if crate::send_browser_key(&target, &key, true).is_ok() {
                                    *guard.held_key.borrow_mut() = Some((target, key));
                                }
                                let weak = Rc::downgrade(&guard);
                                guard.after(options.key_hold_ms, move || {
                                    if let Some(guard) = weak.upgrade()
                                        && let Some((target, key)) =
                                            guard.held_key.borrow_mut().take()
                                    {
                                        let _ = crate::send_browser_key(&target, &key, false);
                                    }
                                    done.emit(());
                                });
                                return;
                            }
                        }
                        done.emit(());
                    });
                }
            };
            attempt();
            if !started.get() {
                *guard.retry.borrow_mut() = Some(Interval::new(options.retry_ms, attempt));
            }
        });
        guard
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::NoExtension;

    fn tour(steps: Vec<Step>) -> Rc<Excursion> {
        Rc::new(Excursion {
            version: 1,
            id: "demo".into(),
            title: "Demo".into(),
            repeat: false,
            cursor: None,
            steps,
        })
    }

    fn reduce(
        runtime: Rc<Runtime<NoExtension>>,
        action: RuntimeAction<NoExtension>,
    ) -> Rc<Runtime<NoExtension>> {
        runtime.reduce(action)
    }

    #[test]
    fn runtime_starts_steps_and_ignores_stale_completions() {
        let runtime = Rc::new(Runtime::new(Some(tour(vec![
            Step::say("a"),
            Step::say("b"),
        ]))));
        let engine = runtime.engine.as_ref().expect("engine");
        assert_eq!(engine.state(), PlaybackState::Waiting);
        let first = engine.pending().expect("token");
        let runtime = reduce(runtime, RuntimeAction::Command(PlaybackCommand::Next));
        assert_eq!(runtime.hurry, Some(first));
        let runtime = reduce(runtime, RuntimeAction::Complete(first));
        assert_eq!(runtime.hurry, None);
        let engine = runtime.engine.as_ref().expect("engine");
        assert_eq!(engine.index(), 1);
        let second = engine.pending().expect("next token");
        let same = reduce(runtime.clone(), RuntimeAction::Complete(first));
        assert!(Rc::ptr_eq(&same, &runtime));
        let runtime = reduce(runtime, RuntimeAction::Complete(second));
        assert_eq!(
            runtime.engine.as_ref().map(PlaybackEngine::state),
            Some(PlaybackState::Finished)
        );
    }

    #[test]
    fn speed_is_clamped_and_kept_when_reloading() {
        let mut runtime = Rc::new(Runtime::new(Some(tour(vec![Step::say("a")]))));
        for _ in 0..20 {
            runtime = reduce(runtime, RuntimeAction::Command(PlaybackCommand::Faster));
        }
        assert_eq!(runtime.speed, 8);
        let tour_b = tour(vec![Step::say("b")]);
        let runtime = reduce(runtime, RuntimeAction::Load(Some(tour_b.clone())));
        assert_eq!(runtime.speed, 8);
        let same = reduce(runtime.clone(), RuntimeAction::Load(Some(tour_b)));
        assert!(Rc::ptr_eq(&same, &runtime));
        let mut runtime = runtime;
        for _ in 0..20 {
            runtime = reduce(runtime, RuntimeAction::Command(PlaybackCommand::Slower));
        }
        assert_eq!(runtime.speed, 1);
    }

    #[test]
    fn explicit_delays_ignore_speed_and_hurry_skips_them() {
        let options = PlaybackOptions::default();
        let mut step = Step::<NoExtension>::say("a");
        assert_eq!(step_delay(&step, &options, 2.0, false), 2_500);
        assert_eq!(step_delay(&step, &options, 0.5, false), 10_000);
        step.delay_ms = Some(300);
        assert_eq!(step_delay(&step, &options, 4.0, false), 300);
        assert_eq!(step_delay(&step, &options, 1.0, true), 0);
    }
}
