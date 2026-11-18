# Navigation

Enable `navigation` on `konnektoren-yew` to use the components through its
prelude. The components use the shared i18n context supplied by the app's i18n provider.

Import `styles/crates/navigation/navigation.css` alongside the shared Konnektoren
styles. Enable `csr` or `ssr` for the corresponding rendering mode.

`NavigationBar`, `NavigationHeader`, and `NavigationMenu` accept optional `classes`
for app-specific styling. Their BEM classes remain available for shared overrides.
Use the existing brand, tools, actions, and item slots for app-specific content.

`ResponsiveNavigation` combines the header and bottom layouts with the same
items and provides brand, tools, labels, active-route, and styling slots.
