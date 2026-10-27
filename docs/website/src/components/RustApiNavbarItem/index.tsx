import React, { type ReactNode } from 'react';
import DefaultNavbarItem from '@theme/NavbarItem/DefaultNavbarItem';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import { useDocsVersionCandidates } from '@docusaurus/plugin-content-docs/client';

// The Rust API reference is rustdoc's own HTML, served from static/ per docs version
// (scripts/run-rustdoc.js), outside the docs plugins. This link opens the reference of the
// version being browsed, else the preferred or newest one, and the working copy's for a release
// that has no Rust binding. `customFields.rustApiVersions` maps a version name to its URL.
type Props = { label?: string; isDropdownItem?: boolean; className?: string };

export default function RustApiNavbarItem({ label = 'Rust API', ...props }: Props): ReactNode {
  const { siteConfig } = useDocusaurusContext();
  const urls = (siteConfig.customFields?.rustApiVersions ?? {}) as Record<string, string>;
  const candidates = useDocsVersionCandidates('default');
  const url = candidates.map((version) => urls[version.name]).find(Boolean) ?? urls.current;
  if (!url) {
    return null;
  }
  // `pathname://` leaves the client router: the target is a static page.
  return <DefaultNavbarItem {...props} label={label} href={`pathname://${url}`} />;
}
