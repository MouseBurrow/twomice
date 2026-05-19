# TwoMice Frontend Design Spec

**Date:** 2026-05-19
**Status:** Confirmed — ready for implementation

---

## 1. Overview

TwoMice is an anonymous, mouse-themed Reddit clone. The frontend is a complete visual redesign targeting a warm, earthy aesthetic with strong geometric personality. Every element — from comment bubbles to header shapes — is deliberately non-generic.

**Stack:** React 19 + TypeScript + Vite + SCSS. No component library.

**Theme switching:** CSS custom properties on `[data-theme][data-mode]` attributes on `<html>`. Zero JS overhead at render time. State stored in `localStorage`.

---

## 2. Terminology

| Generic term | TwoMice term |
|---|---|
| Subreddit / community | **Mischief Board** (or just **board**) |
| URL prefix | `b/boardname` |
| Post | **Nib** |
| Comment | **Squeak** |
| Reply to a comment | **Echo** |
| Upvote / downvote | **Vote** |
| Subscribe | **Join** |

---

## 3. Typography

| Role | Font | Usage |
|---|---|---|
| Brand / headings | Fredoka One | Logo, board names, post titles, nib titles |
| UI / body | Inter | All UI text, metadata, labels |
| Numbers / scores | Space Grotesk | Vote counts, stats |

---

## 4. Shape Language

TwoMice uses two distinct shape categories. Structural chrome (headers, buttons, tabs) uses clip-path geometry. Content areas (comment bubbles, inputs) use border-radius.

### Clip-path shapes

| Element | `clip-path` value | Used on |
|---|---|---|
| Chamfered header | `polygon(0 0, 100% 0, 100% 78%, 94% 100%, 0 100%)` | Post header, board header, settings header, create-nib header |
| Arrow tag (left) | `polygon(0 0, 100% 0, 92% 100%, 0 100%)` | Board tag, nav breadcrumb tags |
| Arrow button (right) | `polygon(0 0, 100% 0, 94% 100%, 0 100%)` | Reply button, Join button, Post Nib button |
| Parallelogram tab | `polygon(8% 0%, 100% 0%, 92% 100%, 0% 100%)` | Sort tabs (Hot / New / Top / Old) |

### Border-radius shapes

| Element | `border-radius` value | Notes |
|---|---|---|
| Comment / echo bubble | `0 16px 16px 16px` | Sharp top-left only — all depths, no exceptions |
| Title / body input | `0 8px 8px 8px` | Mirrors bubble shape |
| Vote pill (post header) | `99px` | Full pill: ▲ score ▼ |
| Board stat container | `6px` | Slight rounding |
| Nav search bar | `99px` | Pill shape |
| Nav user pill | `99px` | Avatar + handle |
| Notification bell | `50%` | Circle |
| Tag chips | `3px` | Subtle rounding |

### Vote stripe

A vertical bar in the vote column whose **height and colour intensity both scale with score**:

- Score ≥ 100: full height (≈32px), `linear-gradient(#8b4513, #c8761a)` — deep brown to amber
- Score 20–99: medium height, `linear-gradient(#a05828, #d49060)`
- Score 5–19: short, flat `#c8a880`
- Score < 5: stub (4–6px), `#e0cdb8` — barely visible

---

## 5. Themes

Theme switching uses `data-theme` + `data-mode` on `<html>`. Example: `<html data-theme="fieldmouse" data-mode="light">`.

### Standard themes (4 × 3 modes = 12 variants)

| Theme | Personality | Key colours |
|---|---|---|
| **Fieldmouse** | Warm, earthy — the default | `#faf5ee` bg, `#8b4513` accent (saddle brown), `#c8761a` accent2 |
| **Midnight** | Deep purple-blue, nocturnal | `#0d0b18` bg, `#7c6fcf` accent |
| **Urban Rat** | Cool grey concrete | `#1e1e1e` bg, `#9ca3af` accent |
| **Pinewood** | Forest green, mossy | `#1a2218` bg, `#4a7c59` accent |

Each theme has **light / mid / dark** variants. The same hue family, shifting surface brightness:
- **Light** — pale background, dark text
- **Mid** — medium-depth background, balanced contrast
- **Dark** — near-black background, light text

### Contrast / specialty themes (3 variants)

| Theme | Description |
|---|---|
| **Stark Light** | Pure white, black text, no warmth — maximum legibility |
| **Stark Dark** | Pure black, white text — true OLED dark |
| **Goldenrod** | High-saturation amber on dark — golden accent, strong contrast |

### CSS custom property schema

```css
[data-theme="fieldmouse"][data-mode="light"] {
  --bg-primary:    #faf5ee;
  --bg-surface:    #ffffff;
  --bg-elevated:   #fdf0d8;
  --border:        #e8ddd0;
  --border-soft:   #f0e8dc;
  --text-primary:  #1a0e04;
  --text-muted:    #a09080;
  --text-faint:    #c0b0a0;
  --accent:        #8b4513;
  --accent2:       #c8761a;
  --accent-bg:     #f5ede0;
  --dash:          #ddd0c0;
  --dash-strong:   #c8b89e;
  --stripe-top:    #8b4513;
  --stripe-bottom: #c8761a;
}
```

---

## 6. Global Shell — Navigation Bar

Two-row structure, always visible at the top of every page.

### Primary row

| Slot | Element | Notes |
|---|---|---|
| Left | Brand `🐭 TwoMice` | Fredoka One, accent colour, links to home |
| Center | Search bar (pill) | Placeholder "search nibs, boards, squeaks…" · ⌘K shortcut hint · `#faf5ee` bg |
| Right | Notification bell | Circle button · unread badge (count) in accent orange |
| Right | User pill | `avatar · anon_xxxx · ▾` — opens dropdown with profile/settings/logout |

### Secondary row

| Slot | Element |
|---|---|
| Left | Breadcrumb trail (`/ b/schemes / nib`) — current board in accent colour |
| Right | Pinned board quick-links — pill chips, active board highlighted |

The secondary row is `#faf5ee` background with a subtle bottom border, sitting beneath the white primary row.

### Anonymous identity

Each user sets a short handle once in settings (e.g. `anon_4f9a`). It appears in the nav user pill and profile page only — **never shown to other users**.

---

## 7. Pages

### 7.1 Home Page

**Layout:** full-width cinematic banner → 3-column post grid → live anonymous ticker

#### Banner (featured post)
- Dark `linear-gradient(160deg, #2a1a0a, #1a0e06)` background (Fieldmouse theme). Other themes use their own dark-variant colours for this section — the banner is always dark regardless of the user's mode setting, for cinematic contrast.
- Inner gradient overlay: `linear-gradient(0deg, rgba(8,4,0,0.88) 0%, rgba(26,14,6,0.15) 70%)`
- Shows: board tag, post title (Fredoka One, large), vote count + squeak count + age
- Height: ~120px in desktop layout

#### 3-column grid
- Three equal columns separated by 1px `#e8e0d4` hairline dividers
- Each column: 2 post cards stacked, divided by a 1px bottom border
- Post card structure: vote number · board label · title · squeak count + age

#### Live anonymous ticker
- Dark `#1e1208` strip at the bottom, `border-top: 1px solid rgba(200,118,26,0.2)`
- Shows board-level activity only — **no usernames, no individual actions**
- Example items: `b/schemes — 14 squeaks in 5 min` · `b/cheeseboard trending ↑200%` · `421 browsing now`
- Green pulse dot next to "LIVE" label
- Can be disabled in settings

---

### 7.2 Board Page (`b/boardname`)

**Layout:** chamfered header → sort bar → nib feed

#### Board header (chamfered)
- Same `clip-path: polygon(0 0, 100% 0, 100% 78%, 94% 100%, 0 100%)` as post header
- Contains: "Mischief Board" label · board name (Fredoka One, 18px) · description text · stats bar
- Stats bar (white card with border, slight rounding): **members · online now · nibs**
- Chamfered arrow Join button on the right of the stats row

#### Sort bar
- `#f5ede0` background · parallelogram tabs (Hot / New / Top)
- `+ Post Nib` chamfered arrow button on the right

#### Nib feed
- One nib per row, separated by dashed rules
- Each row: vote column (▲ stripe score ▼) + nib body
- Nib body: title (Fredoka One) · optional 1–2 line body preview · metadata row
- Metadata: squeak count (accent colour) · age · optional tag chips

---

### 7.3 Post Page

**Layout:** chamfered header → post body → sort bar → comment thread

#### Post header (chamfered)
- `clip-path: polygon(0 0, 100% 0, 100% 78%, 94% 100%, 0 100%)`
- `#fdf0d8` background
- Board arrow tag: `clip-path: polygon(0 0, 100% 0, 92% 100%, 0 100%)`
- Title in Fredoka One (15px)
- Vote row: pill cluster `▲ score ▼` using CSS triangles + border separators · squeak count · age · chamfered `+ Squeak` button

#### Post body
- Padding `9px 14px 7px` · text lines
- `border-bottom: 2px solid var(--accent)` — 2px accent line closes off the post content

#### Sort bar
- Identical to board page sort bar
- Shows squeak count as label on the left

#### Comment thread

**Thread structure:**
- All top-level squeaks sit at the same indent (`padding-left: 10px`)
- Echoes indent right: each nesting level adds 14px
- Echoes-of-echoes add another 14px — unlimited nesting depth
- **Dashed rule** (`border-top: 1px dashed var(--dash-strong)`) appears **only between top-level squeak blocks** — never inside a thread

**Vote column (per comment):**
```
▲  (CSS triangle up)
│  vote stripe bar (height + intensity = score)
N  (score number, Space Grotesk)
▼  (CSS triangle down)
```
Active/upvoted up-arrow fills `var(--accent)`.

**Speech bubbles:**
- `border-radius: 0 16px 16px 16px` — sharp top-left only, all depths
- Depth-1 echo bubbles: slightly warmer background tint (`#fdfaf4`)
- Depth-2+ echo bubbles: `border-left: 2px solid rgba(200,118,26,0.2)` + warm bg (`#faf5ec`)

**Bubble contents:**
- Header row: `anonymous` (italic, muted) · timestamp (right-aligned, faint)
- Body: content text
- Action row: `↩ echo` · `share` · `▲ collapse` (right-aligned)

**Collapsed state:**
- `opacity: 0.70`
- Body and action row hidden
- Shows `▼ expand` in accent colour **only if the squeak has at least one echo**
- No expand toggle if the squeak has no replies

---

### 7.4 Profile Page

**Layout:** chamfered header → tab bar → nib/squeak feed

#### Profile header (chamfered)
- Avatar circle · anonymous handle (Fredoka One) · join date · "edit name" button
- Stats bar (white card): **nibs posted · squeaks made · votes received · boards joined**

#### Tab bar
- Three tabs: **My Nibs · My Squeaks · Saved**
- Active tab: accent colour text + `border-bottom: 2px solid var(--accent)`

#### Feed
- Same nib row pattern as board page
- Shows board name as a coloured label per row (since profile spans all boards)

> **Saved tab:** Since TwoMice is anonymous with no server-side user accounts, saved items are stored in `localStorage` only. Clearing browser data clears saves. This is intentional — no server-side persistence for saved state.

---

### 7.5 Settings Page

**Layout:** chamfered header → grouped setting rows

#### Appearance group
- **Theme** — 7 colour swatches in a small grid. Active swatch gets an accent border.
- **Mode** — Light / Mid / Dark pill selector

#### Feed group
Toggle rows (sliding pill toggle, on = accent fill):
- Show body preview in feeds
- Auto-collapse low-vote squeaks
- Show live ticker on home

#### Identity group
- **Anonymous handle** — editable text field, note that it's never shown to others
- **Pinned boards** — manage link (opens board picker)

#### Data group (separated, red accent)
- **Clear vote history** — destructive, red outline button

---

### 7.6 Create Nib Page

**Layout:** chamfered header → form

#### Form fields

| Field | Required | Component |
|---|---|---|
| Board | Yes | Chip selector — pinned boards first, active chip fills accent, `+ more` expands |
| Title | Yes | Input with `border-radius: 0 8px 8px 8px` (mirrors bubble shape) |
| Body | No | Textarea with light formatting toolbar (B / I / U / list / link) |
| Tags | No | Freeform tag chips, inline `+ add tag…` prompt |

#### Submit row
- `cancel` — plain muted text left
- `Post Nib` — chamfered arrow button right (`clip-path: polygon(0 0, 100% 0, 94% 100%, 0 100%)`)
- Button dims (`background: var(--border)`) when title is empty

---

## 8. Reusable Component Patterns

### Vote column
```
[v-up]           ← CSS triangle, fills accent when active
[vote-stripe]    ← 4px wide pill, height + gradient = score
[vote-num]       ← Space Grotesk, accent colour
[v-down]         ← CSS triangle, muted by default
```
Used on: post header (pill variant), nib feed rows, comment rows.

### CSS triangles
```css
/* Up arrow */
.v-up::after {
  content: '';
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
  border-bottom: 5px solid var(--text-muted);
}
.v-up.active::after { border-bottom-color: var(--accent); }

/* Down arrow */
.v-down::after {
  content: '';
  border-left: 4px solid transparent;
  border-right: 4px solid transparent;
  border-top: 5px solid var(--text-faint);
}
```

### Dashed separator
```css
.rule { border-top: 1px dashed var(--dash); }
.rule.strong { border-color: var(--dash-strong); }
```
Used: between top-level squeaks on post page, between nibs on board/profile feed.

### Chamfered header
```css
.page-header {
  clip-path: polygon(0 0, 100% 0, 100% 78%, 94% 100%, 0 100%);
  background: var(--bg-elevated);
}
```
Used on every page's top section.

### Arrow button
```css
.btn-arrow {
  clip-path: polygon(0 0, 100% 0, 94% 100%, 0 100%);
  background: var(--accent);
  color: #fff;
}
```
Used: + Squeak, + Join, + Post Nib, Post Nib submit.

### Parallelogram sort tab
```css
.sort-tab {
  clip-path: polygon(8% 0%, 100% 0%, 92% 100%, 0% 100%);
  background: var(--bg-primary);
}
.sort-tab.active { background: var(--accent); color: #fff; }
```

---

## 9. Emoji Policy

- `🐭` used as the brand mark in the nav only
- `🐀` may be used as an alternative brand mark
- No decorative emojis anywhere else in the UI

---

## 10. Mockup Reference Files

All mockups are in `.superpowers/brainstorm/28399-1779176761/content/`:

| File | Contents |
|---|---|
| `final-showcase.html` | Home page + Post page — confirmed final |
| `board-page.html` | Board page — confirmed final |
| `shell-and-pages.html` | Nav bar + Profile + Settings + Create Nib — confirmed final |
| `post-final-v6.html` | Post page detail (latest iteration) |
| `theme-matrix-v2.html` | All 4 standard themes × 3 modes |
| `contrast-themes.html` | Stark Light, Stark Dark, Goldenrod |
