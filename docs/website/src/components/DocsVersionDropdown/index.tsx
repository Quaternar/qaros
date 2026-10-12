import React, { useEffect, type ReactNode } from 'react';
import DocsVersionDropdownNavbarItem, {
  type Props as DropdownProps,
} from '@theme/NavbarItem/DocsVersionDropdownNavbarItem';
import {
  useActiveDocContext,
  useActivePlugin,
  useDocsPreferredVersion,
  useVersions,
} from '@docusaurus/plugin-content-docs/client';

// The guides (`default`) and the generated API reference (`api`) are separate
// docs instances versioned in lockstep (scripts/docs-versions.js). The theme's
// version dropdown serves one instance; this one serves whichever instance the
// current page belongs to, and makes the version being browsed the preferred
// version of the other instance too, so version-aware navbar links (`doc`,
// `docSidebar`) into it stay on the same release.
const API_PLUGIN_ID = 'api';
const GUIDES_PLUGIN_ID = 'default';

type Props = Partial<DropdownProps>;

export default function DocsVersionDropdown(props: Props): ReactNode {
  const activePlugin = useActivePlugin({ failfast: false });
  const pluginId = activePlugin?.pluginId === API_PLUGIN_ID ? API_PLUGIN_ID : GUIDES_PLUGIN_ID;
  const otherPluginId = pluginId === API_PLUGIN_ID ? GUIDES_PLUGIN_ID : API_PLUGIN_ID;
  const { activeVersion } = useActiveDocContext(pluginId);
  const otherVersions = useVersions(otherPluginId);
  const { savePreferredVersionName } = useDocsPreferredVersion(otherPluginId);
  const activeName = activeVersion?.name;

  useEffect(() => {
    if (activeName && otherVersions.some((version) => version.name === activeName)) {
      savePreferredVersionName(activeName);
    }
  }, [activeName, otherVersions, savePreferredVersionName]);

  return (
    <DocsVersionDropdownNavbarItem
      items={[]}
      dropdownItemsBefore={[]}
      dropdownItemsAfter={[]}
      {...props}
      docsPluginId={pluginId}
    />
  );
}
