"use client";

import FadeIn from "@/components/FadeIn";

const rows = [
  {
    feature: "Agents supported",
    lesearch: "7+ — any CLI agent",
    others: "1–2 specific agents",
  },
  {
    feature: "Multi-machine",
    lesearch: "SSH + VNC + Headscale VPN",
    others: "Single machine only",
  },
  {
    feature: "GUI access (VNC)",
    lesearch: "Full desktop view from phone",
    others: "Terminal only",
  },
  {
    feature: "Open source",
    lesearch: "LeCoder — fully open, MIT",
    others: "Archived or closed",
  },
  {
    feature: "Secure setup",
    lesearch: "Guided — SSH hardening + isolation",
    others: "DIY, no guidance",
  },
];

function Check() {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" style={{ flexShrink: 0, marginTop: "2px" }}>
      <circle cx="8" cy="8" r="7.5" stroke="#3F3F46" />
      <path d="M4.5 8L7 10.5L11.5 6" stroke="#FAFAFA" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

function Cross() {
  return (
    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" style={{ flexShrink: 0, marginTop: "2px" }}>
      <circle cx="8" cy="8" r="7.5" stroke="#27272A" />
      <path d="M5.5 5.5L10.5 10.5M10.5 5.5L5.5 10.5" stroke="#3F3F46" strokeWidth="1.5" strokeLinecap="round" />
    </svg>
  );
}

export default function Comparison() {
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
          Why LeSearch
        </p>
        <h2
          style={{
            fontSize: "clamp(32px, 4.5vw, 52px)",
            fontWeight: "800",
            letterSpacing: "-0.035em",
            lineHeight: "1.08",
            color: "#FAFAFA",
            margin: "0 0 20px",
            maxWidth: "560px",
          }}
        >
          Built for how power users actually work
        </h2>
        <p
          style={{
            fontSize: "17px",
            lineHeight: "1.65",
            color: "#71717A",
            maxWidth: "480px",
            margin: "0 0 48px",
          }}
        >
          Other tools connect one agent to one machine. LeSearch connects
          everything to everything — securely, from any device.
        </p>
      </FadeIn>

      {/* Install callout */}
      <FadeIn delay={80}>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "16px",
            padding: "20px 28px",
            border: "1px solid #27272A",
            borderRadius: "12px",
            background: "#0D0D0F",
            marginBottom: "16px",
            flexWrap: "wrap",
          }}
        >
          <div
            style={{
              fontFamily:
                "'SF Mono', 'Fira Code', 'Fira Mono', 'Roboto Mono', monospace",
              fontSize: "14px",
              color: "#e0e7ff",
              flex: 1,
              minWidth: "240px",
            }}
          >
            <span style={{ color: "#52525B", marginRight: "12px" }}>$</span>
            npx lecoder-mconnect
          </div>
          <span style={{ fontSize: "12px", color: "#52525B", whiteSpace: "nowrap" }}>
            One command. Any machine.
          </span>
        </div>
      </FadeIn>

      {/* Comparison table */}
      <FadeIn delay={160}>
        <div
          style={{
            border: "1px solid #27272A",
            borderRadius: "16px",
            overflow: "hidden",
          }}
        >
          {/* Header */}
          <div
            style={{
              display: "grid",
              gridTemplateColumns: "1fr 1fr 1fr",
              background: "#18181B",
              borderBottom: "1px solid #27272A",
            }}
          >
            {[
              { label: "Feature", muted: true },
              { label: "LeSearch AI", muted: false },
              { label: "Others", muted: true },
            ].map((col, i) => (
              <div
                key={col.label}
                style={{
                  padding: "16px 24px",
                  fontSize: "12px",
                  fontWeight: col.muted ? "500" : "700",
                  color: col.muted ? "#52525B" : "#FAFAFA",
                  letterSpacing: "0.06em",
                  textTransform: "uppercase",
                  borderLeft: i > 0 ? "1px solid #27272A" : "none",
                }}
              >
                {col.label}
              </div>
            ))}
          </div>

          {/* Rows */}
          {rows.map((row, i) => (
            <div
              key={row.feature}
              style={{
                display: "grid",
                gridTemplateColumns: "1fr 1fr 1fr",
                borderBottom: i < rows.length - 1 ? "1px solid #18181B" : "none",
                background: "#0D0D0F",
              }}
            >
              <div
                style={{
                  padding: "20px 24px",
                  fontSize: "14px",
                  fontWeight: "500",
                  color: "#A1A1AA",
                }}
              >
                {row.feature}
              </div>
              <div
                style={{
                  padding: "20px 24px",
                  borderLeft: "1px solid #18181B",
                  display: "flex",
                  alignItems: "flex-start",
                  gap: "10px",
                }}
              >
                <Check />
                <span style={{ fontSize: "14px", color: "#FAFAFA", lineHeight: "1.5" }}>
                  {row.lesearch}
                </span>
              </div>
              <div
                style={{
                  padding: "20px 24px",
                  borderLeft: "1px solid #18181B",
                  display: "flex",
                  alignItems: "flex-start",
                  gap: "10px",
                }}
              >
                <Cross />
                <span style={{ fontSize: "14px", color: "#52525B", lineHeight: "1.5" }}>
                  {row.others}
                </span>
              </div>
            </div>
          ))}
        </div>
      </FadeIn>
    </section>
  );
}
