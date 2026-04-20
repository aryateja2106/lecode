import type { Metadata } from "next";
import { Toaster } from "sonner";
import "./globals.css";

export const metadata: Metadata = {
  title: "LeSearch AI — Every Agent. Every Machine. One Terminal.",
  description:
    "Control Claude Code, Codex, Cursor, Amp, Droid, and Gemini from your phone. LeSearch AI is the agent infrastructure platform that connects all your AI coding agents across all your machines — securely, from any device. Open source core: LeCoder MConnect.",
  keywords: [
    "LeSearch AI",
    "LeCoder",
    "MConnect",
    "agent infrastructure",
    "terminal on phone",
    "agent orchestration",
    "remote agent access",
    "Claude Code mobile",
    "Codex mobile",
    "Cursor Agent",
    "Amp coding agent",
    "Droid Factory AI",
    "Gemini CLI",
    "AI coding agents",
    "SSH terminal mobile",
    "VNC remote desktop",
    "multi-machine agent",
    "agent harness",
    "OpenClaw alternative",
    "Omnara alternative",
    "mobile coding terminal",
    "remote AI agent control",
    "agent monitoring",
    "coding from phone",
  ],
  authors: [{ name: "Arya Teja Rudraraju", url: "https://aryateja.com" }],
  creator: "LeSearch AI",
  publisher: "LeSearch AI",
  metadataBase: new URL("https://lesearch.ai"),
  alternates: {
    canonical: "https://lesearch.ai",
  },
  openGraph: {
    type: "website",
    locale: "en_US",
    url: "https://lesearch.ai",
    title: "LeSearch AI — Every Agent. Every Machine. One Terminal.",
    description:
      "Control Claude Code, Codex, Cursor, Amp, and 7+ AI coding agents from your phone. Secure multi-machine orchestration with SSH, VNC, and VPN. Open source.",
    siteName: "LeSearch AI",
  },
  twitter: {
    card: "summary_large_image",
    title: "LeSearch AI — Every Agent. Every Machine. One Terminal.",
    description:
      "Control 7+ AI coding agents from your phone. Multi-machine orchestration. Open source. Built in San Francisco.",
    creator: "@aryateja_r",
    site: "@aryateja_r",
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-video-preview": -1,
      "max-image-preview": "large",
      "max-snippet": -1,
    },
  },
  verification: {
    google: "pending",
  },
  category: "technology",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  const organizationSchema = {
    "@context": "https://schema.org",
    "@type": "Organization",
    name: "LeSearch AI",
    url: "https://lesearch.ai",
    logo: "https://lesearch.ai/favicon.svg",
    description:
      "Agent infrastructure platform for power users who run AI coding agents across multiple machines. Control Claude Code, Codex, Cursor, Amp, Droid, and Gemini from any device.",
    founder: {
      "@type": "Person",
      name: "Arya Teja Rudraraju",
      url: "https://aryateja.com",
      jobTitle: "Founder & Agentic Engineer",
    },
    foundingDate: "2025",
    foundingLocation: {
      "@type": "Place",
      name: "San Francisco, CA",
    },
    sameAs: [
      "https://github.com/aryateja2106/lecoder-mconnect",
      "https://x.com/aryateja_r",
      "https://github.com/aryateja2106",
    ],
  };

  const softwareSchema = {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    name: "LeCoder MConnect",
    operatingSystem: "macOS, Linux, Windows, iOS, Android",
    applicationCategory: "DeveloperApplication",
    description:
      "Open-source CLI and mobile app for controlling AI coding agents (Claude Code, Codex, Cursor, Amp, Droid, Gemini) from any device via secure tunneling.",
    url: "https://lesearch.ai",
    downloadUrl: "https://www.npmjs.com/package/lecoder-mconnect",
    softwareVersion: "0.1.11",
    author: {
      "@type": "Organization",
      name: "LeSearch AI",
    },
    offers: [
      {
        "@type": "Offer",
        price: "0",
        priceCurrency: "USD",
        description: "Free tier: 10 sessions per month",
      },
      {
        "@type": "Offer",
        price: "20",
        priceCurrency: "USD",
        description: "Pro: Unlimited sessions",
      },
    ],
  };

  return (
    <html lang="en">
      <head>
        <link rel="icon" href="/favicon.svg" type="image/svg+xml" />
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify(organizationSchema),
          }}
        />
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify(softwareSchema),
          }}
        />
      </head>
      <body>
        {children}
        <Toaster
          theme="dark"
          position="bottom-right"
          toastOptions={{
            style: {
              background: "#18181B",
              border: "1px solid #27272A",
              color: "#FAFAFA",
            },
          }}
        />
      </body>
    </html>
  );
}
