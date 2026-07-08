// Shared, live-syncing store for the selected Developer Guide code language.
//
// Design note: we deliberately do NOT depend on Docusaurus's internal tab-group
// context (its public export changed across 3.x). Instead every language-aware
// component on the page — <CodeTabs> and <LanguageSwitcher> — reads and writes a
// single persisted value and subscribes to a change event, so they all live-sync
// on the current page AND persist site-wide across navigation.
//
// We reuse Docusaurus's own persistence key (`docusaurus.tab.language`) so that
// any remaining native `<Tabs groupId="language">` still pick up the choice on
// their next mount.

export type LanguageValue = 'c' | 'csharp';

export const LANGUAGE_LABELS: Record<string, string> = {
  c: 'C',
  csharp: 'C#',
  // Future languages are a one-line addition here (e.g. cpp: 'C++', unity: 'Unity').
};

export const DEFAULT_LANGUAGE: LanguageValue = 'c';

// Matches Docusaurus's localStorage key for a tab group with groupId="language".
const STORAGE_KEY = 'docusaurus.tab.language';
const EVENT_NAME = 'qaros:language-change';

function isBrowser(): boolean {
  return typeof window !== 'undefined';
}

export function getLanguage(): LanguageValue {
  if (!isBrowser()) return DEFAULT_LANGUAGE;
  try {
    const stored = window.localStorage.getItem(STORAGE_KEY);
    if (stored && stored in LANGUAGE_LABELS) return stored as LanguageValue;
  } catch {
    // localStorage can throw (privacy mode / disabled) — fall through to default.
  }
  return DEFAULT_LANGUAGE;
}

export function setLanguage(value: LanguageValue): void {
  if (!isBrowser()) return;
  try {
    window.localStorage.setItem(STORAGE_KEY, value);
  } catch {
    // ignore persistence failures; still notify listeners for the live session.
  }
  window.dispatchEvent(new CustomEvent<LanguageValue>(EVENT_NAME, { detail: value }));
}

// Subscribe to changes (both same-tab via our custom event and cross-tab via the
// native `storage` event). Returns an unsubscribe function.
export function subscribeLanguage(listener: (value: LanguageValue) => void): () => void {
  if (!isBrowser()) return () => {};
  const onCustom = (e: Event) => {
    const detail = (e as CustomEvent<LanguageValue>).detail;
    if (detail) listener(detail);
  };
  const onStorage = (e: StorageEvent) => {
    if (e.key === STORAGE_KEY && e.newValue && e.newValue in LANGUAGE_LABELS) {
      listener(e.newValue as LanguageValue);
    }
  };
  window.addEventListener(EVENT_NAME, onCustom);
  window.addEventListener('storage', onStorage);
  return () => {
    window.removeEventListener(EVENT_NAME, onCustom);
    window.removeEventListener('storage', onStorage);
  };
}
