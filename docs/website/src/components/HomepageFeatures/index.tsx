import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Extend, Don’t Rewrite',
    Svg: require('@site/static/img/feature-extend.svg').default,
    description: (
      <>
        Add AR/VR support to your existing 3D applications without major code
        rewrites. QarOS plugs into what you already have.
      </>
    ),
  },
  {
    title: 'Low-Latency Streaming',
    Svg: require('@site/static/img/feature-stream.svg').default,
    description: (
      <>
        Remote-render on powerful workstations and stream to a wide range of
        AR/VR devices with low latency.
      </>
    ),
  },
  {
    title: 'Multi-User Collaboration',
    Svg: require('@site/static/img/feature-collaborate.svg').default,
    description: (
      <>
        Bring up to eight users into a shared, synchronized space across
        different platforms and devices.
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
