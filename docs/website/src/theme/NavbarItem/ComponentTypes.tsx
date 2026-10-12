// Registers the site's own navbar item types (`custom-*`) next to the theme's.
import ComponentTypes from '@theme-original/NavbarItem/ComponentTypes';
import DocsVersionDropdown from '@site/src/components/DocsVersionDropdown';

export default {
  ...ComponentTypes,
  'custom-docsVersionDropdown': DocsVersionDropdown,
};
