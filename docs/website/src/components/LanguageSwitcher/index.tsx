import React, { type ReactNode, useEffect, useState } from 'react';
import {
  DEFAULT_LANGUAGE,
  LANGUAGE_LABELS,
  type LanguageValue,
  getLanguage,
  setLanguage,
  subscribeLanguage,
} from '../CodeTabs/languageStore';
import styles from './styles.module.css';

// A visible, top-of-page selector for the Developer Guide code language. It reads
// and writes the SAME persisted choice the <CodeTabs> blocks use (see
// ../CodeTabs/languageStore), so selecting a language here live-updates every code
// tab on the page and persists across navigation.
//
// `languages` defaults to the full label set; pass a subset for pages that only
// show a couple of languages.
export default function LanguageSwitcher({
  languages,
}: {
  languages?: LanguageValue[];
}): ReactNode {
  const values = (languages ?? (Object.keys(LANGUAGE_LABELS) as LanguageValue[]));

  const [selected, setSelected] = useState<LanguageValue>(DEFAULT_LANGUAGE);

  useEffect(() => {
    setSelected(getLanguage());
    return subscribeLanguage(setSelected);
  }, []);

  return (
    <div className={styles.switcher} role="group" aria-label="Choose code language">
      <span className={styles.label}>Language:</span>
      <div className={styles.options}>
        {values.map((value) => {
          const isActive = value === selected;
          return (
            <button
              key={value}
              type="button"
              aria-pressed={isActive}
              className={isActive ? `${styles.option} ${styles.optionActive}` : styles.option}
              onClick={() => setLanguage(value)}
            >
              {LANGUAGE_LABELS[value] ?? value}
            </button>
          );
        })}
      </div>
    </div>
  );
}
