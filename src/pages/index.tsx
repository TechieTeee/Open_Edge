// pages/index.tsx

import Head from 'next/head';
import styles from 'styles/Home.module.css';
import { useRouter } from 'next/router';

function HomePage() {
  const router = useRouter();

  const handleSignInClick = () => {
    // Navigate to the Sign In page or use the appropriate route
    router.push('/signin');
  };

  return (
    <div className={styles.container}>
      {/* Header */}
      <header className={styles.header}>
        <div className="flex justify-between items-center py-4">
          <div className="text-2xl font-bold text-blue-500">Open Edge</div>
          <button
            onClick={handleSignInClick}
            className="text-blue-500 hover:underline rounded-full py-2 px-4 bg-blue-100"
          >
            Sign In
          </button>
        </div>
      </header>

      {/* Main Content */}
      <main className={styles.main}>
        {/* Hero Image */}
        <div className="mb-8">
          <img
            src="/Open_Edge_Logo.png" // Path to your new image
            alt="Open Edge Hero Image"
            className="w-full h-auto"
          />
        </div>

        {/* Problem Statement, How Open Edge Solves the Problem, Key Features, Use Cases */}
        <section className="mb-8">
          <h2 className="text-3xl font-bold mb-4">Problem Statement</h2>
          <p>The dynamic growth of the cloud computing and big data industry, anticipated to reach a valuation between $308.26 billion and $349.56 billion by 2024, brings forth a set of intricate challenges. As organizations grapple with the unprecedented scale of data, issues such as scalability, security, and data governance emerge. Balancing the need for scalable infrastructure with cost optimization, addressing the complexities of data management, and navigating regulatory landscapes require innovative solutions. Furthermore, a shortage of skilled professionals and interoperability challenges add layers of complexity. Overcoming these challenges is pivotal for organizations to harness the full potential of cloud computing and big data, fostering a future where data-driven decision-making is not only powerful but also secure and efficient.</p>
        </section>

        <section className="mb-8">
          <h2 className="text-3xl font-bold mb-4">How Open Edge Solves the Problem</h2>
          <p>
            Open Edge is at the forefront of addressing the challenges inherent in the cloud computing and big data industry. By providing a decentralized, secure, and scalable platform, Open Edge empowers organizations to overcome the hurdles of traditional cloud infrastructures. Through its innovative architecture, Open Edge not only ensures scalability and efficient data management but also prioritizes security and compliance. The platform's decentralized nature minimizes single points of failure and enhances data resilience. Open Edge acts as a catalyst for seamless collaboration and data sharing while mitigating the complexities associated with interoperability. By fostering a community-driven approach, Open Edge revolutionizes the landscape of cloud computing and big data, enabling organizations to navigate challenges and unlock the full potential of their data resources</p>
        </section>

        <section>
          <h2 className="text-3xl font-bold mb-4">Key Features</h2>
          <ul className="list-disc pl-4">
            <li>Decentralized Architecture</li>
            <li>Scalability</li>
            <li>Enhanced Security</li>
            <li>Interoperability</li>
            <li>Community-Driven Development</li>
            <li>Efficient Data Management</li>
            <li>Smart Contract Support</li>
            <li>Cross-Platform Compatibility</li>
            <li>Data Resilience</li>
            <li>Compliance and Regulatory Alignment</li>
          </ul>
        </section>

        {/* Use Cases */}
        <section className="mb-8">
          <h2 className="text-3xl font-bold mb-4">Use Cases</h2>
          <p>
            Open Edge is versatile and can be applied to various scenarios. Some notable use cases include:
          </p>
          <ul className="list-disc pl-4">
            <li>Easy and Secure Management of Legal Filings and Access Control</li>
            <li>Secure and Transparent Supply Chain Management Quality Management</li>
            <li>Efficient and Trustworthy Data Sharing in Healthcare (EHR, Credentials, etc)</li>
          </ul>
        </section>

        {/* Try It Out */}
        <section className="mb-8">
          <h2 className="text-3xl font-bold mb-4">Try It Out</h2>
          <p>
            Experience the power of Open Edge firsthand. Click the button below to try our demo.
          </p>
          <button
            onClick={() => router.push('/DemoPage')}
            className="bg-blue-500 text-white px-4 py-2 rounded-full mt-4 inline-block"
          >
            Demo
          </button>
        </section>
      </main>

      {/* Footer */}
      <footer className={styles.footer}>
        <p className="text-sm text-gray-500">Open Edge © 2024</p>
      </footer>
    </div>
  );
}

export default HomePage;