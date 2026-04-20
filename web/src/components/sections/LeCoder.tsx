"use client";

import FadeIn from "@/components/FadeIn";

const tools = [
  {
    name: "LeCoder CLI",
    badge: "npm",
    command: "npx lecoder-mconnect",
    description:
      "The core tunnel and terminal sharing layer. Creates a secure Cloudflare-backed tunnel from your machine to any device in seconds.",
    detail: "Open source · MIT",
  },
  {
    name: "MConnect iOS",
    badge: "App Store",
    command: "Terminal in your pocket",
    description:
      "A real terminal on your iPhone. QR code pairing, no port forwarding required. Built specifically for agent orchestration, not repurposed from SSH clients.",
    detail: "App Store pending · TestFlight live",
  },
  {
    name: "Vox",
    badge: "model",
    command: "Natural language to shell",
    description:
      "A fine-tuned NL-to-shell model running locally on your machines. Tell your agent what you want in plain English. Vox handles the command translation.",
    detail: "Qwen3.5 fine-tune · Ollama compatible",
  },
];

export default function LeCoder() {
  return (
    <section
      id="lecoder"
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
          Open Source
        </p>
        <h2
          style={{
            fontSize: "clamp(32px, 4.5vw, 52px)",
            fontWeight: "800",
            letterSpacing: "-0.035em",
            lineHeight: "1.08",
            color: "#FAFAFA",
            margin: "0 0 20px",
            maxWidth: "580px",
          }}
        >
          Built on Open Source
        </h2>
        <p
          style={{
            fontSize: "17px",
            lineHeight: "1.65",
            color: "#71717A",
            maxWidth: "520px",
            margin: "0 0 72px",
          }}
        >
          LeCoder is our open-source agent orchestration layer. Use it
          standalone or as the foundation for a full LeSearch AI setup.
        </p>
      </FadeIn>

      <div
        style={{
          display: "grid",
          gridTemplateColumns: "repeat(auto-fit, minmax(280px, 1fr))",
          gap: "16px",
          marginBottom: "16px",
        }}
      >
        {tools.map((tool, i) => (
          <FadeIn key={tool.name} delay={i * 80}>
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
                  display: "flex",
                  alignItems: "center",
                  gap: "10px",
                  marginBottom: "20px",
                }}
              >
                <span
                  style={{
                    fontSize: "15px",
                    fontWeight: "700",
                    color: "#FAFAFA",
                    letterSpacing: "-0.01em",
                  }}
                >
                  {tool.name}
                </span>
                <span
                  style={{
                    padding: "2px 8px",
                    fontSize: "10px",
                    fontWeight: "500",
                    color: "#52525B",
                    border: "1px solid #27272A",
                    borderRadius: "4px",
                    letterSpacing: "0.04em",
                    textTransform: "uppercase",
                  }}
                >
                  {tool.badge}
                </span>
              </div>

              <div
                style={{
                  padding: "10px 14px",
                  background: "#18181B",
                  border: "1px solid #27272A",
                  borderRadius: "8px",
                  marginBottom: "20px",
                  fontFamily:
                    "'SF Mono', 'Fira Code', 'Fira Mono', 'Roboto Mono', monospace",
                  fontSize: "12px",
                  color: "#e0e7ff",
                }}
              >
                {tool.command}
              </div>

              <p
                style={{
                  fontSize: "14px",
                  lineHeight: "1.7",
                  color: "#71717A",
                  margin: "0 0 20px",
                }}
              >
                {tool.description}
              </p>

              <p style={{ fontSize: "12px", color: "#3F3F46", margin: "0" }}>
                {tool.detail}
              </p>
            </div>
          </FadeIn>
        ))}
      </div>

      {/* GitHub CTA */}
      <FadeIn delay={280}>
        <div
          style={{
            display: "flex",
            alignItems: "center",
            gap: "16px",
            padding: "24px 32px",
            border: "1px solid #27272A",
            borderRadius: "14px",
            background: "#0D0D0F",
            marginTop: "16px",
            flexWrap: "wrap",
          }}
        >
          <div style={{ flex: 1 }}>
            <p
              style={{
                fontSize: "15px",
                fontWeight: "600",
                color: "#FAFAFA",
                margin: "0 0 4px",
                letterSpacing: "-0.01em",
              }}
            >
              github.com/aryateja2106/lecoder-mconnect
            </p>
            <p style={{ fontSize: "13px", color: "#52525B", margin: "0" }}>
              MIT license · Issues and PRs welcome
            </p>
          </div>
          <a
            href="https://github.com/aryateja2106/lecoder-mconnect"
            target="_blank"
            rel="noopener noreferrer"
            style={{
              display: "inline-flex",
              alignItems: "center",
              padding: "10px 20px",
              fontSize: "14px",
              fontWeight: "600",
              color: "#FAFAFA",
              background: "transparent",
              border: "1px solid #27272A",
              borderRadius: "8px",
              textDecoration: "none",
              whiteSpace: "nowrap",
              transition: "border-color 0.15s",
            }}
            onMouseEnter={(e) =>
              ((e.target as HTMLElement).style.borderColor = "#52525B")
            }
            onMouseLeave={(e) =>
              ((e.target as HTMLElement).style.borderColor = "#27272A")
            }
          >
            Star on GitHub
          </a>
        </div>
      </FadeIn>
    </section>
  );
}
