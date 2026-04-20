"use client";

import FadeIn from "@/components/FadeIn";

const steps = [
  {
    step: "01",
    title: "Set Up Securely",
    body: "SSH hardening, agent isolation, proper credential scoping. One command gets you from zero to a production-grade agent environment. No security shortcuts.",
    detail: "npx lecoder-mconnect --setup",
  },
  {
    step: "02",
    title: "Connect from Anywhere",
    body: "QR code pairing. No port forwarding. No VPN configuration. Cloudflare tunnel handles the routing. Open MConnect on your phone and scan.",
    detail: "Works on iOS — App Store pending",
  },
  {
    step: "03",
    title: "See Everything",
    body: "Terminal and GUI access in one interface. SSH for shell access. VNC for desktop view. Full visibility into what every agent is doing on every machine.",
    detail: "SSH + VNC + Cloudflare",
  },
  {
    step: "04",
    title: "Orchestrate",
    body: "Run multiple agents across machines. Monitor, control, and coordinate from any device. If something needs attention, you see it. If it can run unattended, it does.",
    detail: "Multi-machine agent control",
  },
];

export default function HowItWorks() {
  return (
    <section
      style={{
        padding: "112px 24px",
        maxWidth: "1120px",
        margin: "0 auto",
      }}
    >
      <FadeIn>
        <p
          style={{
            fontSize: "12px",
            fontWeight: "500",
            color: "#52525B",
            letterSpacing: "0.1em",
            textTransform: "uppercase",
            marginBottom: "16px",
          }}
        >
          How It Works
        </p>
        <h2
          style={{
            fontSize: "clamp(32px, 4.5vw, 52px)",
            fontWeight: "800",
            letterSpacing: "-0.035em",
            lineHeight: "1.08",
            color: "#FAFAFA",
            margin: "0 0 72px",
            maxWidth: "480px",
          }}
        >
          Four steps to full agent control
        </h2>
      </FadeIn>

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fit, minmax(240px, 1fr))",
          gap: "24px",
        }}
      >
        {steps.map((s, i) => (
          <FadeIn key={s.step} delay={i * 80}>
            <div
              style={{
                padding: "32px 28px",
                border: "1px solid #27272A",
                borderRadius: "14px",
                background: "#0D0D0F",
                height: "100%",
              }}
            >
              <div
                style={{
                  fontSize: "12px",
                  fontWeight: "600",
                  color: "#3F3F46",
                  letterSpacing: "0.06em",
                  marginBottom: "24px",
                  fontFamily: "'SF Mono', 'Fira Code', 'Fira Mono', 'Roboto Mono', monospace",
                }}
              >
                {s.step}
              </div>
              <h3
                style={{
                  fontSize: "17px",
                  fontWeight: "700",
                  color: "#FAFAFA",
                  letterSpacing: "-0.02em",
                  lineHeight: "1.3",
                  margin: "0 0 12px",
                }}
              >
                {s.title}
              </h3>
              <p
                style={{
                  fontSize: "14px",
                  lineHeight: "1.7",
                  color: "#71717A",
                  margin: "0 0 20px",
                }}
              >
                {s.body}
              </p>
              <div
                style={{
                  display: "inline-block",
                  padding: "4px 10px",
                  fontSize: "11px",
                  color: "#A1A1AA",
                  background: "#18181B",
                  border: "1px solid #27272A",
                  borderRadius: "6px",
                  fontFamily: "'SF Mono', 'Fira Code', 'Fira Mono', 'Roboto Mono', monospace",
                }}
              >
                {s.detail}
              </div>
            </div>
          </FadeIn>
        ))}
      </div>
    </section>
  );
}
