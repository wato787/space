# Accessibility manual checklist

## Keyboard navigation

- [ ] Tab order follows the visual flow (skip link -> nav -> main)
- [ ] Focus ring is visible on links, buttons, and form fields
- [ ] Shift + Tab moves backward without traps

## Controls and state

- [ ] "Mark as favorite" toggles `aria-pressed` and label text
- [ ] "Show details" toggles `aria-expanded` and the hidden panel

## Forms and errors

- [ ] Submitting without email shows an error message
- [ ] Error message is announced (aria-live) and input has `aria-invalid`
- [ ] Submitting a valid email clears the error

## Live regions

- [ ] Status update button announces the new message

## Landmarks and structure

- [ ] Skip link moves focus to the `main` region
- [ ] Headings follow a logical order (h1 -> h2)
