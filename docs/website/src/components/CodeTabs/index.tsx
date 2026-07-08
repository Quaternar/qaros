import React, {
  type ReactNode,
  isValidElement,
  useEffect,
  useState,
} from 'react';
import {
  DEFAULT_LANGUAGE,
  LANGUAGE_LABELS,
  type LanguageValue,
  getLanguage,
  setLanguage,
  subscribeLanguage,
} from './languageStore';
import styles from './styles.module.css';

// Marker wrapper for one language's content inside <CodeTabs>. It renders
// nothing on its own — <CodeTabs> reads its `value` prop and children.
export function Lang(_props: {
  value: LanguageValue;
  children: ReactNode;
}): ReactNode {
  return null;
}

type LangElement = React.ReactElement<{ value: LanguageValue; children: ReactNode }>;

function collectLangs(children: ReactNode): LangElement[] {
  const result: LangElement[] = [];
  React.Children.forEach(children, (child) => {
    if (isValidElement(child) && (child.type as any) === Lang) {
      result.push(child as LangElement);
    }
  });
  return result;
}

// Thin, self-syncing wrapper that presents one code block per language and keeps
// its choice in sync with every other <CodeTabs> / <LanguageSwitcher> on the page
// and site-wide (see ./languageStore). Adding a language is a one-line change in
// LANGUAGE_LABELS.
export default function CodeTabs({ children }: { children: ReactNode }): ReactNode {
  const langs = collectLangs(children);
  const available = langs.map((l) => l.props.value);

  // SSR renders the default; the stored choice is applied after mount to avoid
  // hydration mismatch.
  const [selected, setSelected] = useState<LanguageValue>(DEFAULT_LANGUAGE);

  useEffect(() => {
    setSelected(getLanguage());
    return subscribeLanguage(setSelected);
  }, []);

  // Fall back to the first available language if the chosen one isn't provided
  // on this particular CodeTabs instance.
  const active = available.includes(selected) ? selected : available[0];
  const activeLang = langs.find((l) => l.props.value === active);

  return (
    <div className={styles.codeTabs}>
      <div className={styles.tabBar} role="tablist" aria-label="Code language">
        {langs.map((l) => {
          const value = l.props.value;
          const isActive = value === active;
          return (
            <button
              key={value}
              type="button"
              role="tab"
              aria-selected={isActive}
              className={isActive ? `${styles.tab} ${styles.tabActive}` : styles.tab}
              onClick={() => setLanguage(value)}
            >
              {LANGUAGE_LABELS[value] ?? value}
            </button>
          );
        })}
      </div>
      <div className={styles.panel} role="tabpanel">
        {activeLang?.props.children}
      </div>
    </div>
  );
}
