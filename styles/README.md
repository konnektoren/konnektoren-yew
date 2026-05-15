# Styles Architecture

The style system follows **Atomic Design** layered on **TailwindCSS v4** utilities and **DaisyUI** components. All layers compose upward — atoms know nothing above them, molecules reference atoms, and so on.

```
styles/
├── tailwind.css          # Tailwind directives
├── vendors.css           # Third-party overrides
├── base/                 # HTML element defaults
├── atoms/                # Single-purpose utilities (@utility)
├── molecules/            # Composed reusable patterns (@utility)
├── templates/            # Domain layout patterns (@utility)
├── layouts/              # Page structural shells (@utility)
├── components/           # App-specific components (BEM classes)
└── themes/               # DaisyUI theme definitions
```

---

## Layers

### base/
Global HTML element resets and typographic defaults (`body`, headings, `a`, `img`). No class names — just tag selectors.

### atoms/
Smallest indivisible style units. Each file covers one concern:

| File | Utilities |
|------|-----------|
| `button.css` | `btn-primary`, `btn-secondary`, `btn-tertiary`, `btn-material`, `btn-gradient` |
| `typography.css` | `text-adaptive-xl/lg/md/base/sm/xs` |
| `spacing.css` | `gap-adaptive`, `gap-compact`, `padding-adaptive`, `padding-card`, `margin-section` |
| `layout.css` | `container-sm/md/lg`, `container-challenge`, `flex-center`, `flex-between`, `grid-2col` |
| `grids.css` | `grid-auto-fit`, `grid-auto-fill`, `card-grid`, `card-grid-single/double/triple` |
| `surfaces.css` | `surface`, `surface-elevated`, `surface-bordered`, `surface-gradient`, `surface-interactive` |
| `interactions.css` | `interactive`, `interactive-scale`, `hover-lift`, `active-press` |
| `animations.css` | `animate-slide-in`, `animate-fade-in`, `animate-pulse-subtle` |
| `states.css` | `loading-overlay`, `empty-state`, `error-state`, `success-state` |
| `icons.css` | `icon`, `icon-sm/md/lg`, `icon-interactive` |
| `badges.css` | `badge-counter` |

**Rule:** atoms must be domain-agnostic. No challenge-, product-, or feature-specific names here.

### molecules/
Multi-atom patterns that form a reusable UI unit. Each molecule composes atoms and DaisyUI primitives:

| File | Utilities |
|------|-----------|
| `card.css` | `card-base`, `card-interactive`, `card-header`, `card-title`, `card-content`, `card-footer`, `value-display`, `value-display-large`, `value-display-label` |
| `button-group.css` | `button-group` |
| `input-field.css` | `input-field`, `input-label`, `input-hint`, `input-error` |
| `badge-group.css` | `badge-group`, `badge-interactive`, `badge-status`, `badge-success/error/warning/info-soft` |
| `navigation-item.css` | `nav-item`, `nav-item-active`, `nav-item-icon`, `nav-item-label`, `nav-item-horizontal` |
| `progress.css` | `progress-track`, `progress-fill`, `progress-fill-gradient`, `progress-fill-primary/success/warning/error` |
| `list.css` | `list-base`, `list-spaced`, `list-item-base`, `list-item-interactive` |
| `table.css` | `table-base`, `table-header`, `table-header-cell`, `table-row`, `table-cell`, `table-row-success/error/highlighted` |
| `media.css` | `media-container`, `media-image`, `media-image-hover`, `media-image-standalone`, `media-icon` |
| `ad-card.css` | `ad-card-base`, `ad-card-interactive`, `ad-button` |

### templates/
Domain-specific compositional patterns for common page types. Build on atoms, molecules, and DaisyUI.

| File | Utilities |
|------|-----------|
| `challenge-layout.css` | `challenge-layout-content`, `challenge-question-card`, `challenge-options-grid-2x2`, `challenge-option-card`, `challenge-help-card` |
| `result-layout.css` | `result-summary-card`, `result-header`, `result-header-excellent/good/fair/poor`, `result-content` |

### layouts/
Page-level structural shells — the outermost wrappers a page renders into.

| File | Utilities |
|------|-----------|
| `page.css` | `page-layout`, `page-container`, `page-title`, `page-content` |

### components/
Application-specific components using **BEM** (Block__Element--Modifier). Each component owns its styles and composes molecules/atoms via `@apply`.

```
components/
├── challenge/       # challenge, question, options, results, dialog
├── certificates/    # certificate, achievement
├── analytics/       # success-rate, average-time-taken (BEM + molecule @apply)
├── marketplace/     # product, cart, wallet, token
├── settings/        # language, theme, level, design selectors
├── ads/             # advertisement, blog-ad, buy-me-coffee
├── tour/            # tooltip, tour-config, tour-button
└── *.css            # navigation, timer, badge, progress-bar, logo, …
```

BEM convention: `.block__element--modifier`

```css
.challenge { }              /* block */
.challenge__header { }      /* element */
.challenge__header--compact { } /* modifier (if needed) */
```

Components apply molecule utilities to stay DRY:
```css
.success-rate { @apply card-base; }
.success-rate__gauge { @apply progress-track; }
.success-rate__gauge-fill { @apply progress-fill-gradient; }
```

### themes/
DaisyUI theme definitions via `@plugin "daisyui/theme"`. Two themes ship: `light` and `dark`. Colors use the OKLCH color space for perceptual uniformity.

---

## Composition Rules

1. **Atoms** — pure utilities only, no domain names, no `@apply` of other custom utilities
2. **Molecules** — `@apply` atoms and DaisyUI classes; add hover/active via `@layer utilities`
3. **Templates** — `@apply` molecules and atoms for domain layout patterns
4. **Layouts** — `@apply` atoms and molecules for page shells
5. **Components** — BEM class names, `@apply` molecules/atoms/templates to stay DRY

## Adding a new component

1. Create `components/<name>.css` with BEM classes
2. `@apply` existing molecule utilities where possible
3. Add `@import "./<name>.css"` to `components/index.css`
4. If the pattern is reusable across components, extract it to `molecules/`

## Adding a new molecule

1. Create `molecules/<name>.css` with `@utility` definitions
2. Compose from `atoms/` and DaisyUI only
3. Add hover/focus/active state in `@layer utilities { .utility-name:hover { } }`
4. Add `@import "./<name>.css"` to `molecules/index.css`
