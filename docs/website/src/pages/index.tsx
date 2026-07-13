import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useBaseUrl from '@docusaurus/useBaseUrl';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
  const logoUrl = useBaseUrl('/img/brand/qaros-vertical-split.png');
  return (
    <header className={clsx('hero', styles.heroBanner)}>
      <div className={clsx('container', styles.heroInner)}>
        <img
          className={styles.heroLogo}
          src={logoUrl}
          alt="QAROS"
          width={180}
          height={180}
        />
        <span className={styles.heroEyebrow}>Documentation &amp; Knowledge Base</span>
        <Heading as="h1" className={styles.heroTitle}>
          AR/VR for the 3D software you already have
        </Heading>
        <p className={styles.heroSubtitle}>
          QAROS is Quaternar&rsquo;s AR/VR middleware. It streams your existing
          application to headsets, tablets, and phones, composites several apps
          into one shared space, and handles discovery, security, and multi-user
          sessions &mdash; with no engine migration and no reimplementation.
        </p>
        <p className={styles.heroTagline}>Don&rsquo;t Rewrite. Extend.</p>
        <div className={styles.buttons}>
          <Link
            className="button button--primary button--lg"
            to="/docs/intro">
            Start with the concepts
          </Link>
          <Link
            className="button button--secondary button--lg"
            to="/docs/developer-guide/getting-started">
            Developer Guide
          </Link>
          <Link
            className={clsx(
              'button button--outline button--secondary button--lg',
              styles.heroButtonOutline,
            )}
            to="/api/qar-streaming-c/">
            C API Reference
          </Link>
        </div>
      </div>
    </header>
  );
}

type PathItem = {
  audience: string;
  title: string;
  description: string;
  to: string;
  cta: string;
};

const PathList: PathItem[] = [
  {
    audience: 'Users',
    title: 'Join a shared session',
    description:
      'Bring a HoloLens, Android phone or tablet, VR headset, or desktop viewer into a shared session with a short pairing code, and learn how the room and its shared panels behave.',
    to: '/docs/user-guide/qaros-world',
    cta: 'Open the User Guide',
  },
  {
    audience: 'Developers',
    title: 'Integrate the API',
    description:
      'Add QAROS to your application with the C or C#/.NET SDK: onboarding, sessions, streaming rendered frames, GUI panels, and app volumes. Every snippet is backed by a compiled example.',
    to: '/docs/developer-guide/getting-started',
    cta: 'Open the Developer Guide',
  },
  {
    audience: 'Administrators & Operators',
    title: 'Install & run a Hub',
    description:
      'Install, secure, configure, and maintain a QAROS Hub: system requirements, the runtime launcher and its configuration, networking and Hub federation, the security model, and device management.',
    to: '/docs/operator-guide/overview',
    cta: 'Open the Operator Guide',
  },
];

function HomepagePaths(): ReactNode {
  return (
    <section className={styles.paths}>
      <div className="container">
        <div className={styles.pathsHeader}>
          <Heading as="h2">Find your path</Heading>
          <p>
            This site is where teams learn how QAROS works. For product details
            and pricing, visit{' '}
            <Link href="https://quaternar.com">quaternar.com</Link>.
          </p>
        </div>
        <div className="row">
          {PathList.map((item) => (
            <div key={item.title} className="col col--4">
              <div className={styles.pathCard}>
                <span className={styles.pathAudience}>{item.audience}</span>
                <Heading as="h3" className={styles.pathTitle}>
                  {item.title}
                </Heading>
                <p className={styles.pathDescription}>{item.description}</p>
                <Link className={styles.pathLink} to={item.to}>
                  {item.cta} &rarr;
                </Link>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title} — Don’t Rewrite. Extend.`}
      description="QAROS is Quaternar’s AR/VR middleware for existing 3D applications: stream to headsets, tablets, and phones, composite apps into one shared space, and run secure multi-user sessions.">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <HomepagePaths />
      </main>
    </Layout>
  );
}
