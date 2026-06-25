import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://edithatogo.github.io',
  base: '/open_social_data/',
  integrations: [
    mdx(),
    sitemap(),
    starlight({
      title: 'Open Social Data',
      description: 'Legal NZ documentation portal for Open Social Data.',
      sidebar: [
        { label: 'Start', items: ['index', 'getting-started', 'docs-tooling-audit'] },
        { label: 'Reference', items: ['cli-reference', 'providers', 'dataset-packs', 'catalog', 'validation'] },
        { label: 'Project', items: ['roadmap-status', 'release', 'provenance-archival'] },
      ],
    }),
  ],
});
