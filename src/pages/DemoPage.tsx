// pages/DemoPage.tsx

import Head from 'next/head';
import { useDropzone } from 'react-dropzone';
import styles from 'styles/Home.module.css';

// Updated ProgressBar component
const ProgressBar = ({ percentage, label }) => (
  <div className="flex items-center mb-2">
    <span className="w-20 mr-2 text-right">{label}</span>
    <div className="flex-1 bg-gray-200 h-6 rounded-full overflow-hidden">
      <div
        className="h-full bg-blue-500 rounded-full"
        style={{ width: `${percentage}%` }}
      />
    </div>
  </div>
);

function DemoPage() {
  // UseDropzone hook for handling file drops
  const { acceptedFiles, getRootProps, getInputProps } = useDropzone();

  return (
    <div className={styles.container}>
      {/* Header */}
      <header className={styles.header}>
        <div className="flex justify-between items-center py-4">
          <div className="text-2xl font-bold text-blue-500">Open Edge</div>
          <button className="text-blue-500 hover:underline rounded-full py-2 px-4 bg-blue-100">Sign In</button>
        </div>
      </header>

      {/* Main Content */}
      <main className={styles.main}>
        {/* Demo-specific content */}
        <section className="mb-8">
          <h2 className="text-3xl font-bold mb-4">Open Edge Demo</h2>
          {/* Add your demo content here */}
          <p>
            Welcome to the Open Edge Demo page! This is where you can explore the features and capabilities of Open Edge.
          </p>
          {/* Upload Widget with Drop Area */}
          <div className="mt-4 relative" {...getRootProps()} style={{ backgroundImage: `url('/Open_Edge_Drop_Zone_Image.png')`, backgroundSize: 'contain', backgroundPosition: 'center', backgroundRepeat: 'no-repeat', minHeight: '400px' }}>
            <input {...getInputProps()} />
            <div className="absolute bottom-0 left-0 right-0 text-center mb-4">
              <p className="text-gray-500">Drag 'n' drop files here, or click to select files</p>
              <ul>
                {acceptedFiles.map((file) => (
                  <li key={file.path}>{file.path}</li>
                ))}
              </ul>
            </div>
          </div>
        </section>

        {/* Statistics */}
        <section className="mb-8">
          <h2 className="text-3xl font-bold mb-4">Statistics</h2>
          {/* Example statistics on ICP uptime, availability, and recent activity */}
          <p>
            Uptime: 99.9%
          </p>
          <ProgressBar label="Availability" percentage={80} />
        </section>

        {/* Upload History */}
        <section className="mb-8">
          <h2 className="text-3xl font-bold mb-4">Open Edge Upload History</h2>
          {/* Example upload history component or list */}
          <ul className="list-disc pl-4">
            <li>Feb 05, 2024</li>
            <li>Jan 17, 2024</li>
            <li>Dec 28, 2024</li>
          </ul>
        </section>

        {/* Node Locations */}
        <section className="mb-8">
          <h2 className="text-3xl font-bold mb-4">Node Locations</h2>
          <p>Check Out Nearby Validators: <a href="https://www.google.com/maps" target="_blank" rel="noopener noreferrer">Nearby Validators</a></p>
        </section>
      </main>

      {/* Footer */}
      <footer className={styles.footer}>
        <p className="text-sm text-gray-500">Open Edge © 2024</p>
      </footer>
    </div>
  );
}

export default DemoPage;
