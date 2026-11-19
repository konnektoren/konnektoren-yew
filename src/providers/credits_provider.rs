//! Asset-loading hooks for credits catalogs and license text.
use crate::model::CreditAssetServer;
use konnektoren_asset_loader::{AssetFormat, AssetServer, SourceLayer};
use std::{rc::Rc, sync::Arc};
use yew::prelude::*;

#[derive(Clone)]
struct SourceKey(Rc<dyn SourceLayer>);

impl PartialEq for SourceKey {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

#[hook]
pub(crate) fn use_credit_server(source: Rc<dyn SourceLayer>) -> CreditAssetServer {
    let server = use_memo(SourceKey(source), |source| {
        CreditAssetServer(Rc::new(AssetServer::new(source.0.clone())))
    });
    (*server).clone()
}

type Loaded<T> = Option<Result<Arc<T>, String>>;

#[hook]
pub(crate) fn use_credit_asset<F>(
    server: CreditAssetServer,
    path: AttrValue,
) -> (Loaded<F::Asset>, Callback<MouseEvent>)
where
    F: AssetFormat + Default + 'static,
    F::Asset: Send + Sync + 'static,
{
    let state = use_state(|| None);
    let revision = use_state(|| 0usize);
    #[cfg(feature = "csr")]
    {
        let state = state.clone();
        use_effect_with(
            (server.clone(), path.clone(), *revision),
            move |(server, path, _)| {
                let active = Rc::new(std::cell::Cell::new(true));
                let pending = active.clone();
                let server = server.clone();
                let path = path.to_string();
                state.set(None);
                wasm_bindgen_futures::spawn_local(async move {
                    let loaded = server
                        .0
                        .load(F::default(), path)
                        .await
                        .map_err(|error| error.to_string());
                    if pending.get() {
                        state.set(Some(loaded));
                    }
                });
                move || active.set(false)
            },
        );
    }
    #[cfg(not(feature = "csr"))]
    let _ = (server, path);
    let retry = Callback::from(move |_| revision.set(*revision + 1));
    ((*state).clone(), retry)
}
